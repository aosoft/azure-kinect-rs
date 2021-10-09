use crate::recorder::Error;
use azure_kinect_sys::k4a::*;
use clap::{App, Arg, ArgMatches};
use std::time::Duration;

pub struct Parameter {
    pub list_device: bool,
    pub device_index: u32,
    pub recording_filename: String,
    pub recording_length: Option<Duration>,
    pub device_config: k4a_device_configuration_t,
    pub record_imu: bool,
    pub absolute_exposure_value: Option<i32>,
    pub gain: Option<i32>,
}

impl Parameter {
    pub fn new<'a>() -> Result<Parameter, Error<'a>> {
        let p = Parameter::from(create_app().get_matches());

        if let Ok(r) = p.as_ref() {
            if !r.list_device && r.recording_filename.len() == 0 {
                create_app().print_help();
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
            device_config: k4a_device_configuration_t {
                color_format: format_resolution.0,
                color_resolution: format_resolution.1,
                depth_mode: to_depth_mode(args.value_of("depth-mode").unwrap())?,
                camera_fps: to_frame_rate(args.value_of("rate").unwrap())?,
                synchronized_images_only: false,
                depth_delay_off_color_usec: args
                    .value_of("depth-delay")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0),
                wired_sync_mode: to_external_sync(args.value_of("external-sync").unwrap())?,
                subordinate_delay_off_master_usec: args
                    .value_of("sync-delay")
                    .unwrap_or("0")
                    .parse()
                    .unwrap_or(0),
                disable_streaming_indicator: false,
            },
            record_imu: args
                .value_of("imu")
                .unwrap_or("ON")
                .eq_ignore_ascii_case("ON"),
            absolute_exposure_value: correct_param_range(
                args.value_of("exposure-control"),
                2,
                200000,
            ),
            gain: correct_param_range(args.value_of("gain"), 0, 255),
        };

        if param.device_config.camera_fps == k4a_fps_t_K4A_FRAMES_PER_SECOND_30
            && (param.device_config.depth_mode == k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_UNBINNED
                || param.device_config.color_resolution
                    == k4a_color_resolution_t_K4A_COLOR_RESOLUTION_3072P)
        {
            return Err(Error::ErrorStr(
                "Error: 30 Frames per second is not supported by this camera mode.",
            ));
        }

        if param.device_config.subordinate_delay_off_master_usec > 0
            && param.device_config.wired_sync_mode
                != k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE
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
) -> Result<(k4a_image_format_t, k4a_color_resolution_t), Error<'a>> {
    match value.to_ascii_lowercase().as_str() {
        "3072p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_3072P,
        )),
        "2160p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_2160P,
        )),
        "1536p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1536P,
        )),
        "1440p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1440P,
        )),
        "1080p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1080P,
        )),
        "720p" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
        )),
        "720p_nv12" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_NV12,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
        )),
        "720p_yuy2" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_YUY2,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
        )),
        "off" => Ok((
            k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
        )),
        _ => Err(Error::Error(format!(
            "Unknown color mode specified: {}",
            value
        ))),
    }
}

fn to_depth_mode<'a>(value: &str) -> Result<k4a_depth_mode_t, Error<'a>> {
    match value.to_ascii_uppercase().as_str() {
        "NFOV_2X2BINNED" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_2X2BINNED),
        "NFOV_UNBINNED" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED),
        "WFOV_2X2BINNED" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED),
        "WFOV_UNBINNED" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_UNBINNED),
        "PASSIVE_IR" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_PASSIVE_IR),
        "OFF" => Ok(k4a_depth_mode_t_K4A_DEPTH_MODE_OFF),
        _ => Err(Error::Error(format!(
            "Unknown depth mode specified: {}",
            value
        ))),
    }
}

fn to_frame_rate<'a>(value: &str) -> Result<k4a_fps_t, Error<'a>> {
    match value {
        "30" => Ok(k4a_fps_t_K4A_FRAMES_PER_SECOND_30),
        "15" => Ok(k4a_fps_t_K4A_FRAMES_PER_SECOND_15),
        "5" => Ok(k4a_fps_t_K4A_FRAMES_PER_SECOND_5),
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

fn to_external_sync<'a>(value: &str) -> Result<k4a_wired_sync_mode_t, Error<'a>> {
    match value.to_ascii_lowercase().as_str() {
        "master" => Ok(k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_MASTER),
        "subordinate" => Ok(k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE),
        "sub" => Ok(k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE),
        "standalone" => Ok(k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE),
        _ => Err(Error::Error(format!(
            "Unknown external sync mode specified: {}",
            value
        ))),
    }
}

#[test]
fn conv_param_test() {
    assert!(
        to_format_and_resolution("3072p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_3072P
            )
    );
    assert!(
        to_format_and_resolution("2160p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_2160P
            )
    );
    assert!(
        to_format_and_resolution("1536p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1536P
            )
    );
    assert!(
        to_format_and_resolution("1440p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1440P
            )
    );
    assert!(
        to_format_and_resolution("1080p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1080P
            )
    );
    assert!(
        to_format_and_resolution("720p").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P
            )
    );
    assert!(to_format_and_resolution("720p_nv12").is_ok());
    assert!(to_format_and_resolution("720p_yuy2").is_ok());
    assert!(
        to_format_and_resolution("720p_NV12").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_NV12,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P
            )
    );
    assert!(
        to_format_and_resolution("720p_YUY2").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_YUY2,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P
            )
    );
    assert!(
        to_format_and_resolution("OFF").unwrap()
            == (
                k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
                k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF
            )
    );
    assert!(to_format_and_resolution("asdqv").is_err());

    assert!(
        to_depth_mode("NFOV_2X2BINNED").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_2X2BINNED
    );
    assert!(
        to_depth_mode("NFOV_UNBINNED").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED
    );
    assert!(
        to_depth_mode("WFOV_2X2BINNED").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED
    );
    assert!(
        to_depth_mode("WFOV_UNBINNED").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_UNBINNED
    );
    assert!(to_depth_mode("PASSIVE_IR").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_PASSIVE_IR);
    assert!(to_depth_mode("OFF").unwrap() == k4a_depth_mode_t_K4A_DEPTH_MODE_OFF);
    assert!(to_depth_mode("off").is_ok());
    assert!(to_depth_mode("asdwergsdgsdfds").is_err());

    assert!(to_frame_rate("30").unwrap() == k4a_fps_t_K4A_FRAMES_PER_SECOND_30);
    assert!(to_frame_rate("15").unwrap() == k4a_fps_t_K4A_FRAMES_PER_SECOND_15);
    assert!(to_frame_rate("5").unwrap() == k4a_fps_t_K4A_FRAMES_PER_SECOND_5);
    assert!(to_frame_rate("1").is_err());

    assert!(to_imu_mode("ON").unwrap());
    assert!(!to_imu_mode("OFF").unwrap());
    assert!(to_imu_mode("On").unwrap());
    assert!(to_imu_mode("poasdas").is_err());

    assert!(
        to_external_sync("master").unwrap() == k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_MASTER
    );
    assert!(
        to_external_sync("Subordinate").unwrap()
            == k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE
    );
    assert!(
        to_external_sync("SUB").unwrap() == k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE
    );
    assert!(
        to_external_sync("STANDALONE").unwrap()
            == k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE
    );
    assert!(to_external_sync("as098kasd").is_err());
}
