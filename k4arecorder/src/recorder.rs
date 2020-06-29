use azure_kinect::bindings::k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF;
use azure_kinect::bindings::k4a_depth_mode_t::K4A_DEPTH_MODE_OFF;
use azure_kinect::*;

#[derive(Debug)]
pub(crate) enum Error<'a> {
    ErrorStr(&'a str),
    Error(String),
}

impl std::error::Error for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::ErrorStr(s) => s,
                Error::Error(s) => s,
            }
        )
    }
}

const DEFAULT_EXPOSURE_AUTO: i32 = -12;
const DEFAULT_GAIN_AUTO: i32 = -1;

pub(crate) fn do_recording(
    factory: &FactoryRecord,
    device_index: u32,
    recording_filename: &str,
    device_config: &k4a_device_configuration_t,
    record_imu: bool,
    absoluteExposureValue: i32,
    gain: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let installed_devices = factory.device_get_installed_count();
    if device_index >= installed_devices {
        return Err(Box::new(Error::ErrorStr("Device not found.")));
    }

    let device = match factory.device_open(device_index) {
        Ok(device) => device,
        Err(_) => {
            return Err(Box::new(Error::ErrorStr(
                "Runtime error: k4a_device_open() failed ",
            )))
        }
    };

    let serial_number = device.get_serialnum()?;
    println!("Device serial number: {}", serial_number);
    let version_info = device.get_version()?;
    print!(
        "Device version: {}",
        if version_info.firmware_build == k4a_firmware_build_t::K4A_FIRMWARE_BUILD_RELEASE {
            "Rel"
        } else {
            "Dbg"
        }
    );
    print!(
        "; C: {}.{}.{}",
        version_info.rgb.major, version_info.rgb.minor, version_info.rgb.iteration
    );
    print!(
        "; D: {}.{}.{}[{}.{}]",
        version_info.depth.major,
        version_info.depth.minor,
        version_info.depth.iteration,
        version_info.depth_sensor.major,
        version_info.depth_sensor.minor
    );
    println!(
        "; A: {}.{}.{}",
        version_info.audio.major, version_info.audio.minor, version_info.audio.iteration
    );

    let camera_fps = device_config.camera_fps.get_u32();
    if camera_fps <= 0
        || (device_config.color_resolution == K4A_COLOR_RESOLUTION_OFF
            && device_config.depth_mode == K4A_DEPTH_MODE_OFF)
    {
        return Err(Box::new(Error::ErrorStr(
            "Either the color or depth modes must be enabled to record.",
        )));
    }

    if absoluteExposureValue != DEFAULT_EXPOSURE_AUTO {
        if let Err(_) = device.set_color_control(
            k4a_color_control_command_t::K4A_COLOR_CONTROL_EXPOSURE_TIME_ABSOLUTE,
            k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_MANUAL,
            absoluteExposureValue,
        ) {
            return Err(Box::new(Error::ErrorStr(
                "Runtime error: k4a_device_set_color_control() for manual exposure failed ",
            )));
        }
    } else {
        if let Err(_) = device.set_color_control(
            k4a_color_control_command_t::K4A_COLOR_CONTROL_EXPOSURE_TIME_ABSOLUTE,
            k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_AUTO,
            0,
        ) {
            return Err(Box::new(Error::ErrorStr(
                "Runtime error: k4a_device_set_color_control() for auto exposure failed ",
            )));
        }
    }

    if gain != DEFAULT_GAIN_AUTO {
        if let Err(_) = device.set_color_control(
            k4a_color_control_command_t::K4A_COLOR_CONTROL_GAIN,
            k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_MANUAL,
            gain,
        ) {
            return Err(Box::new(Error::ErrorStr(
                "Runtime error: k4a_device_set_color_control() for manual gain failed ",
            )));
        }
    } else {
        if let Err(_) = device.set_color_control(
            k4a_color_control_command_t::K4A_COLOR_CONTROL_GAIN,
            k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_AUTO,
            0,
        ) {
            return Err(Box::new(Error::ErrorStr(
                "Runtime error: k4a_device_set_color_control() for auto gain failed ",
            )));
        }
    }

    let camera = device.start_cameras(&device_config)?;
    let imu = if record_imu {
        Option::Some(camera.start_imu()?)
    } else {
        Option::None
    };

    println!("Device started");

    let recording = match factory.record_create(recording_filename, &device, &device_config) {
        Ok(recording) => recording,
        Err(_) => {
            return Err(Box::new(Error::Error(format!(
                "Unable to create recording file: {}",
                recording_filename
            ))))
        }
    };

    if imu.is_some() {
        recording.add_imu_track()?;
    }
    recording.write_header()?;

    // Wait for the first capture before starting recording.
    let timeout_sec_for_first_capture = if device_config.wired_sync_mode
        == k4a_wired_sync_mode_t::K4A_WIRED_SYNC_MODE_SUBORDINATE
    {
        println!("[subordinate mode] Waiting for signal from master");
        360
    } else {
        60
    };

    Ok(())
}
