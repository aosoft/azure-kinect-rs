use azure_kinect::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Error<'a> {
    pub message: &'a str,
}

impl std::error::Error for Error<'_> {}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
        return Err(Box::new(Error {
            message: "Device not found.",
        }));
    }

    let device = match factory.device_open(device_index) {
        Ok(device) => device,
        Err(_) => {
            return Err(Box::new(Error {
                message: "Runtime error: k4a_device_open() failed ",
            }))
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

    Ok(())
}
