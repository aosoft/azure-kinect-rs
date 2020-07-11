use azure_kinect::*;
use std::time::{Instant, Duration};
use clap::{App, Arg, ArgMatches, SubCommand};

pub struct Parameter {
    pub list_device: bool,
    pub device_index: u32,
    pub recording_filename: String,
    pub recording_length: Option<Duration>,
    pub device_config: k4a_device_configuration_t,
    pub record_imu: bool,
    pub absoluteExposureValue: i32,
    pub gain: i32,
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter::from(create_app().get_matches())
    }

    fn from<'a>(args: ArgMatches<'a>) -> Parameter {
        Parameter {
            list_device: false,
            device_index: 0,
            recording_filename: "".to_string(),
            recording_length: None,
            device_config: Default::default(),
            record_imu: false,
            absoluteExposureValue: 0,
            gain: 0
        }
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
            .help("Sets the output file")
            .required(true)
            .default_value(("output.mkv")))
}