use crate::recorder::Error;
use clap::{App, Arg, ArgMatches};
use std::time::Duration;
use azure_kinect::*;

pub struct Parameter {
    pub list_device: bool,
    pub device_index: u32,
    pub recording_filename: String,
    pub recording_length: Option<Duration>,
    pub device_config: DeviceConfiguration,
    pub record_imu: bool,
    pub absolute_exposure_value: Option<i32>,
    pub gain: Option<i32>,
}

impl Parameter {
    pub fn new<'a>() -> Result<Parameter, Error<'a>> {
        let p = Parameter::from(create_app().get_matches());

        if let Ok(r) = p.as_ref() {
            if !r.list_device && r.recording_filename.len() == 0 {
                create_app().print_help().or_else(|_| { Err(Error::ErrorStr("err")) })?;
                std::process::exit(1);
            }
        }

        p
    }

    fn from<'a, 'b>(args: ArgMatches<'a>) -> Result<Parameter, Error<'b>> {
        let format_resolution = to_format_and_resolution(args.value_of("color-mode").unwrap())?;
        let param = Parameter {
            list_device: args.is_present("list"),
            device_index: args.value_of("device").unwrap_or("0").parse().unwrap_or(0),
            recording_filename: args.value_of("OUTPUT").unwrap_or("").to_string(),
            recording_length: correct_param::<u64, _, _>(args.value_of("record-length"), |value| {
                Duration::from_secs(std::cmp::max(0, value))
            }),
            device_config: DeviceConfiguration::builder()
                .color_format(format_resolution.0)
                .color_resolution(format_resolution.1)
                .depth_mode(to_depth_mode(args.value_of("depth-mode").unwrap())?)
                .camera_fps(to_frame_rate(args.value_of("rate").unwrap())?)
                .synchronized_images_only(false)
                .depth_delay_off_color_usec(args
                    .value_of("depth-delay")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0))
                .wired_sync_mode(to_external_sync(args.value_of("external-sync").unwrap())?)
                .subordinate_delay_off_master_usec(args
                    .value_of("sync-delay")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0))
                .disable_streaming_indicator(false)
                .build(),
            record_imu: to_imu_mode(args
                .value_of("imu")
                .unwrap_or("ON"))?,
            absolute_exposure_value: correct_param_range(
                args.value_of("exposure-control"),
                2,
                200000,
            ),
            gain: correct_param_range(args.value_of("gain"), 0, 255),
        };

        if param.device_config.camera_fps() == Fps::_30fps
            && (param.device_config.depth_mode() == DepthMode::WFovUnbinned
            || param.device_config.color_resolution()
            == ColorResolution::_3072p)
        {
            return Err(Error::ErrorStr(
                "Error: 30 Frames per second is not supported by this camera mode.",
            ));
        }

        if param.device_config.subordinate_delay_off_master_usec() > 0
            && param.device_config.wired_sync_mode()
            != WiredSyncMode::Subordinate
        {
            return Err(Error::ErrorStr(
                "--sync-delay is only valid if --external-sync is set to Subordinate.",
            ));
        }

        Ok(param)
    }
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("k4arecorder")
        .arg(Arg::with_name("list")
            .long("list")
            .help("List the currently connected K4A devices")
        )
        .arg(Arg::with_name("device")
            .long("device")
            .help("Specify the device index to use")
            .default_value("0"))
        .arg(Arg::with_name("record-length")
            .long("record-length")
            .short("l")
            .help("Limit the recording to N seconds")
            .default_value("infinite"))
        .arg(Arg::with_name("color-mode")
            .long("color-mode")
            .short("c")
            .help("Set the color sensor mode, Available options:\n3072p, 2160p, 1536p, 1440p, 1080p, 720p, 720p_NV12, 720p_YUY2, OFF")
            .default_value("1080p"))
        .arg(Arg::with_name("depth-mode")
            .long("depth-mode")
            .short("d")
            .help("Set the depth sensor mode, Available options:\nNFOV_2X2BINNED, NFOV_UNBINNED, WFOV_2X2BINNED, WFOV_UNBINNED, PASSIVE_IR, OFF")
            .default_value("NFOV_UNBINNED"))
        .arg(Arg::with_name("depth-delay")
            .long("depth-delay")
            .help("Set the time offset between color and depth frames in microseconds\nA negative value means depth frames will arrive before color frames.\nThe delay must be less than 1 frame period.")
            .default_value("0"))
        .arg(Arg::with_name("rate")
            .long("rate")
            .short("r")
            .default_value("30")
            .help("Set the camera frame rate in Frames per Second\nDefault is the maximum rate supported by the camera modes.\nAvailable options: 30, 15, 5"))
        .arg(Arg::with_name("imu")
            .long("imu")
            .help("Set the IMU recording mode (ON, OFF)")
            .default_value("ON"))
        .arg(Arg::with_name("external-sync")
            .long("external-sync")
            .help("Set the external sync mode (Master, Subordinate, Standalone)")
            .default_value("Standalone"))
        .arg(Arg::with_name("sync-delay")
            .long("sync-delay")
            .help("Set the external sync delay off the master camera in microseconds\nThis setting is only valid if the camera is in Subordinate mode.")
            .default_value("0"))
        .arg(Arg::with_name("exposure-control")
            .long("exposure-control")
            .short("e")
            .help("Set manual exposure value from 2 us to 200,000us for the RGB camera (default: auto exposure). This control also supports MFC settings of -11 to 1)."))
        .arg(Arg::with_name("gain")
            .long("gain")
            .short("g")
            .help("Set cameras manual gain. The valid range is 0 to 255. (default: auto)"))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file"))
}

fn correct_param<T: Ord + core::str::FromStr, U: Ord, F: Fn(T) -> U>(
    value: Option<&str>,
    f: F,
) -> Option<U> {
    match value {
        Some(value) => match value.parse() {
            Ok(value) => Some(f(value)),
            Err(_) => None,
        },
        None => None,
    }
}

fn correct_param_range<T: Ord + core::str::FromStr + Copy + Clone>(
    value: Option<&str>,
    min: T,
    max: T,
) -> Option<T> {
    correct_param(value, |value| std::cmp::max(min, std::cmp::min(max, value)))
}

fn to_format_and_resolution<'a>(
    value: &str,
) -> Result<(ImageFormat, ColorResolution), Error<'a>> {
    match value.to_ascii_lowercase().as_str() {
        "3072p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_3072p,
        )),
        "2160p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_2160p,
        )),
        "1536p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_1536p,
        )),
        "1440p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_1440p,
        )),
        "1080p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_1080p,
        )),
        "720p" => Ok((
            ImageFormat::MJPG,
            ColorResolution::_720p,
        )),
        "720p_nv12" => Ok((
            ImageFormat::NV12,
            ColorResolution::_720p,
        )),
        "720p_yuy2" => Ok((
            ImageFormat::YUY2,
            ColorResolution::_720p,
        )),
        "off" => Ok((
            ImageFormat::MJPG,
            ColorResolution::Off,
        )),
        _ => Err(Error::Error(format!(
            "Unknown color mode specified: {}",
            value
        ))),
    }
}

fn to_depth_mode<'a>(value: &str) -> Result<DepthMode, Error<'a>> {
    match value.to_ascii_uppercase().as_str() {
        "NFOV_2X2BINNED" => Ok(DepthMode::NFov2x2Binned),
        "NFOV_UNBINNED" => Ok(DepthMode::NFovUnbinned),
        "WFOV_2X2BINNED" => Ok(DepthMode::WFov2x2Binned),
        "WFOV_UNBINNED" => Ok(DepthMode::WFovUnbinned),
        "PASSIVE_IR" => Ok(DepthMode::PassiveIr),
        "OFF" => Ok(DepthMode::Off),
        _ => Err(Error::Error(format!(
            "Unknown depth mode specified: {}",
            value
        ))),
    }
}

fn to_frame_rate<'a>(value: &str) -> Result<Fps, Error<'a>> {
    match value {
        "30" => Ok(Fps::_30fps),
        "15" => Ok(Fps::_15fps),
        "5" => Ok(Fps::_5fps),
        _ => Err(Error::Error(format!(
            "Unknown frame rate specified: {}",
            value
        ))),
    }
}

fn to_imu_mode<'a>(value: &str) -> Result<bool, Error<'a>> {
    match value.to_ascii_uppercase().as_str() {
        "ON" => Ok(true),
        "OFF" => Ok(false),
        _ => Err(Error::Error(format!(
            "Unknown imu mode specified: {}",
            value
        ))),
    }
}

fn to_external_sync<'a>(value: &str) -> Result<WiredSyncMode, Error<'a>> {
    match value.to_ascii_lowercase().as_str() {
        "master" => Ok(WiredSyncMode::Master),
        "subordinate" => Ok(WiredSyncMode::Subordinate),
        "sub" => Ok(WiredSyncMode::Subordinate),
        "standalone" => Ok(WiredSyncMode::Standalone),
        _ => Err(Error::Error(format!(
            "Unknown external sync mode specified: {}",
            value
        ))),
    }
}

#[test]
fn conv_param_test() {
    assert_eq!(to_format_and_resolution("3072p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_3072p
    ));
    assert_eq!(to_format_and_resolution("2160p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_2160p
    ));
    assert_eq!(to_format_and_resolution("1536p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_1536p
    ));
    assert_eq!(to_format_and_resolution("1440p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_1440p
    ));
    assert_eq!(to_format_and_resolution("1080p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_1080p
    ));
    assert_eq!(to_format_and_resolution("720p").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_720p
    ));
    assert!(to_format_and_resolution("720p_nv12").is_ok());
    assert!(to_format_and_resolution("720p_yuy2").is_ok());
    assert_eq!(to_format_and_resolution("720p_NV12").unwrap(), (
        ImageFormat::NV12,
        ColorResolution::_720p
    ));
    assert_eq!(to_format_and_resolution("720p_YUY2").unwrap(), (
        ImageFormat::YUY2,
        ColorResolution::_720p
    ));
    assert_eq!(to_format_and_resolution("OFF").unwrap(), (
        ImageFormat::MJPG,
        ColorResolution::_Off
    ));
    assert!(to_format_and_resolution("asdqv").is_err());

    assert_eq!(to_depth_mode("NFOV_2X2BINNED").unwrap(), DepthMode::NFov2x2Binned);
    assert_eq!(to_depth_mode("NFOV_UNBINNED").unwrap(), DepthMode::NFovUnbinned);
    assert_eq!(to_depth_mode("WFOV_2X2BINNED").unwrap(), DepthMode::WFov2x2Binned);
    assert_eq!(to_depth_mode("WFOV_UNBINNED").unwrap(), DepthMode::WFovUnbinned);
    assert_eq!(to_depth_mode("PASSIVE_IR").unwrap(), DepthMode::PassiveIr);
    assert_eq!(to_depth_mode("OFF").unwrap(), DepthMode::Off);
    assert!(to_depth_mode("off").is_ok());
    assert!(to_depth_mode("asdwergsdgsdfds").is_err());

    assert_eq!(to_frame_rate("30").unwrap(), Fps::_30fps);
    assert_eq!(to_frame_rate("15").unwrap(), Fps::_15fps);
    assert_eq!(to_frame_rate("5").unwrap(), Fps::_5fps);
    assert!(to_frame_rate("1").is_err());

    assert!(to_imu_mode("ON").unwrap());
    assert!(!to_imu_mode("OFF").unwrap());
    assert!(to_imu_mode("On").unwrap());
    assert!(to_imu_mode("poasdas").is_err());

    assert_eq!(to_external_sync("master").unwrap(), WiredSyncMode::Master);
    assert_eq!(to_external_sync("Subordinate").unwrap(), WiredSyncMode::Subordinate);
    assert_eq!(to_external_sync("SUB").unwrap(), WiredSyncMode::Subordinate);
    assert_eq!(to_external_sync("STANDALONE").unwrap(), WiredSyncMode::Standalone);
    assert!(to_external_sync("as098kasd").is_err());
}
