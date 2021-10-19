use crate::structs::*;
use azure_kinect_sys::k4a::*;

macro_rules! impl_conv_primitive_to_enum {
    ($enum_type:ident, $primitive_type:ident) => {
        impl From<$enum_type> for $primitive_type {
            fn from(s: $enum_type) -> Self {
                s as _
            }
        }

        impl $enum_type {
            #[allow(dead_code)]
            pub(crate) fn from_primitive(s: $primitive_type) -> Self {
                unsafe { std::mem::transmute(s) }
            }
        }
    };
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Verbosity levels of debug messaging"]
pub enum LogLevel {
    #[doc = "< Most severe level of debug messaging."]
    Critical = k4a_log_level_t_K4A_LOG_LEVEL_CRITICAL,
    #[doc = "< 2nd most severe level of debug messaging."]
    Error = k4a_log_level_t_K4A_LOG_LEVEL_ERROR,
    #[doc = "< 3nd most severe level of debug messaging."]
    Warning = k4a_log_level_t_K4A_LOG_LEVEL_WARNING,
    #[doc = "< 2nd least severe level of debug messaging."]
    Info = k4a_log_level_t_K4A_LOG_LEVEL_INFO,
    #[doc = "< Least severe level of debug messaging."]
    Trace = k4a_log_level_t_K4A_LOG_LEVEL_TRACE,
    #[doc = "< No logging is performed"]
    Off = k4a_log_level_t_K4A_LOG_LEVEL_OFF,
}

impl_conv_primitive_to_enum!(LogLevel, k4a_log_level_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DepthMode {
    Off = k4a_depth_mode_t_K4A_DEPTH_MODE_OFF,
    NFov2x2Binned = k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_2X2BINNED,
    NFovUnbinned = k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED,
    WFov2x2Binned = k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED,
    WFovUnbinned = k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_UNBINNED,
    PassiveIr = k4a_depth_mode_t_K4A_DEPTH_MODE_PASSIVE_IR,
}

impl_conv_primitive_to_enum!(DepthMode, k4a_depth_mode_t);

impl DepthMode {
    /// Gets the dimensions of the depth images that the depth camera will produce for a
    /// given depth mode
    pub fn get_dimension(&self) -> Dimension {
        match self {
            DepthMode::NFov2x2Binned => Dimension {
                width: 320,
                height: 288,
            },
            DepthMode::NFovUnbinned => Dimension {
                width: 640,
                height: 576,
            },
            DepthMode::WFov2x2Binned => Dimension {
                width: 512,
                height: 512,
            },
            DepthMode::WFovUnbinned => Dimension {
                width: 1024,
                height: 1024,
            },
            DepthMode::PassiveIr => Dimension {
                width: 1024,
                height: 1024,
            },
            _ => Dimension {
                width: 0,
                height: 0,
            },
        }
    }

    /// Gets the range of values that we expect to see from the depth camera
    /// when using a given depth mode, in millimeters
    pub fn get_range(&self) -> Range<u16> {
        match self {
            DepthMode::NFov2x2Binned => Range::<u16> {
                min: 500,
                max: 5800,
            },
            DepthMode::NFovUnbinned => Range::<u16> {
                min: 500,
                max: 4000,
            },
            DepthMode::WFov2x2Binned => Range::<u16> {
                min: 250,
                max: 3000,
            },
            DepthMode::WFovUnbinned => Range::<u16> {
                min: 250,
                max: 2500,
            },
            _ => Range::<u16> { min: 0, max: 0 },
        }
    }

    /// Gets the expected min/max IR brightness levels that we expect to see
    /// from the IR camera when using a given depth mode
    pub fn get_ir_level(&self) -> Range<u16> {
        match self {
            DepthMode::PassiveIr => Range::<u16> {
                min: 250,
                max: 3000,
            },
            DepthMode::Off => Range::<u16> { min: 0, max: 0 },
            _ => Range::<u16> { min: 0, max: 1000 },
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorResolution {
    Off = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
    _720p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
    _1080p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1080P,
    _1440p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1440P,
    _1536p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1536P,
    _2160p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_2160P,
    _3072p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_3072P,
}

impl_conv_primitive_to_enum!(ColorResolution, k4a_color_resolution_t);

impl ColorResolution {
    /// Gets the dimensions of the color images that the color camera will produce for a
    /// given color resolution
    pub fn get_dimension(&self) -> Dimension {
        match self {
            ColorResolution::_720p => Dimension {
                width: 1280,
                height: 720,
            },
            ColorResolution::_1080p => Dimension {
                width: 1920,
                height: 1080,
            },
            ColorResolution::_1440p => Dimension {
                width: 2560,
                height: 1440,
            },
            ColorResolution::_1536p => Dimension {
                width: 2048,
                height: 1536,
            },
            ColorResolution::_2160p => Dimension {
                width: 3840,
                height: 2160,
            },
            ColorResolution::_3072p => Dimension {
                width: 4096,
                height: 3072,
            },
            _ => Dimension {
                width: 0,
                height: 0,
            },
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Image format type."]
pub enum ImageFormat {
    #[doc = " Color image type MJPG."]
    MJPG = k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
    #[doc = " Color image type NV12."]
    NV12 = k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_NV12,
    #[doc = " Color image type YUY2."]
    YUY2 = k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_YUY2,
    #[doc = " Color image type BGRA32."]
    BGRA32 = k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_BGRA32,
    #[doc = " Depth image type DEPTH16."]
    Depth16 = k4a_image_format_t_K4A_IMAGE_FORMAT_DEPTH16,
    #[doc = " Image type IR16."]
    IR16 = k4a_image_format_t_K4A_IMAGE_FORMAT_IR16,
    #[doc = " Single channel image type CUSTOM8."]
    Custom8 = k4a_image_format_t_K4A_IMAGE_FORMAT_CUSTOM8,
    #[doc = " Single channel image type CUSTOM16."]
    Custom16 = k4a_image_format_t_K4A_IMAGE_FORMAT_CUSTOM16,
    #[doc = " Custom image format."]
    Custom = k4a_image_format_t_K4A_IMAGE_FORMAT_CUSTOM,
}

impl_conv_primitive_to_enum!(ImageFormat, k4a_image_format_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Transformation interpolation type."]
pub enum TransformationInterpolationType {
    #[doc = "< Nearest neighbor interpolation"]
    Nearest = k4a_transformation_interpolation_type_t_K4A_TRANSFORMATION_INTERPOLATION_TYPE_NEAREST,
    #[doc = "< Linear interpolation"]
    Linear = k4a_transformation_interpolation_type_t_K4A_TRANSFORMATION_INTERPOLATION_TYPE_LINEAR,
}

impl_conv_primitive_to_enum!(
    TransformationInterpolationType,
    k4a_transformation_interpolation_type_t
);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fps {
    _5fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_5,
    _15fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_15,
    _30fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
}

impl_conv_primitive_to_enum!(Fps, k4a_fps_t);

impl Fps {
    pub fn get_u32(&self) -> u32 {
        match self {
            Fps::_5fps => 5,
            Fps::_15fps => 15,
            Fps::_30fps => 30,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Color sensor control commands"]
pub enum ColorControlCommand {
    #[doc = " Exposure time setting."]
    ExposureTimeAbsolute = k4a_color_control_command_t_K4A_COLOR_CONTROL_EXPOSURE_TIME_ABSOLUTE,
    #[doc = " Exposure or Framerate priority setting."]
    AutoExposurePriority = k4a_color_control_command_t_K4A_COLOR_CONTROL_AUTO_EXPOSURE_PRIORITY,
    #[doc = " Brightness setting."]
    Brightness = k4a_color_control_command_t_K4A_COLOR_CONTROL_BRIGHTNESS,
    #[doc = " Contrast setting."]
    Contrast = k4a_color_control_command_t_K4A_COLOR_CONTROL_CONTRAST,
    #[doc = " Saturation setting."]
    Saturation = k4a_color_control_command_t_K4A_COLOR_CONTROL_SATURATION,
    #[doc = " Sharpness setting."]
    Sharpness = k4a_color_control_command_t_K4A_COLOR_CONTROL_SHARPNESS,
    #[doc = " White balance setting."]
    WhiteBalance = k4a_color_control_command_t_K4A_COLOR_CONTROL_WHITEBALANCE,
    #[doc = " Backlight compensation setting."]
    BacklightCompensation = k4a_color_control_command_t_K4A_COLOR_CONTROL_BACKLIGHT_COMPENSATION,
    #[doc = " Gain setting."]
    Gain = k4a_color_control_command_t_K4A_COLOR_CONTROL_GAIN,
    #[doc = " Powerline frequency setting."]
    PowerlineFrequency = k4a_color_control_command_t_K4A_COLOR_CONTROL_POWERLINE_FREQUENCY,
}

impl_conv_primitive_to_enum!(ColorControlCommand, k4a_color_control_command_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Calibration types."]
pub enum ColorControlMode {
    #[doc = "< set the associated k4a_color_control_command_t to auto"]
    Auto = k4a_color_control_mode_t_K4A_COLOR_CONTROL_MODE_AUTO,
    #[doc = "< set the associated k4a_color_control_command_t to manual"]
    Manual = k4a_color_control_mode_t_K4A_COLOR_CONTROL_MODE_MANUAL,
}

impl_conv_primitive_to_enum!(ColorControlMode, k4a_color_control_mode_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Synchronization mode when connecting two or more devices together."]
pub enum WiredSyncMode {
    #[doc = "< Neither 'Sync In' or 'Sync Out' connections are used."]
    Standalone = k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE,
    #[doc = "< The 'Sync Out' jack is enabled and synchronization data it driven out the connected wire."]
    Master = k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_MASTER,
    #[doc = "< The 'Sync In' jack is used for synchronization and 'Sync Out' is driven for the next device in the chain."]
    Subordinate = k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_SUBORDINATE,
}

impl_conv_primitive_to_enum!(WiredSyncMode, k4a_wired_sync_mode_t);

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Calibration types."]
pub enum CalibrationType {
    #[doc = "< Calibration type is unknown"]
    Unknown = k4a_calibration_type_t_K4A_CALIBRATION_TYPE_UNKNOWN,
    #[doc = "< Depth sensor"]
    Depth = k4a_calibration_type_t_K4A_CALIBRATION_TYPE_DEPTH,
    #[doc = "< Color sensor"]
    Color = k4a_calibration_type_t_K4A_CALIBRATION_TYPE_COLOR,
    #[doc = "< Gyroscope sensor"]
    Gyro = k4a_calibration_type_t_K4A_CALIBRATION_TYPE_GYRO,
    #[doc = "< Accelerometer sensor"]
    Accel = k4a_calibration_type_t_K4A_CALIBRATION_TYPE_ACCEL,
}

impl_conv_primitive_to_enum!(CalibrationType, k4a_calibration_type_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Calibration model type."]
pub enum CalibrationModelType {
    #[doc = "< Calibration model is unknown"]
    Unknown = k4a_calibration_model_type_t_K4A_CALIBRATION_LENS_DISTORTION_MODEL_UNKNOWN,
    #[doc = "< Deprecated (not supported). Calibration model is Theta (arctan)."]
    Theta = k4a_calibration_model_type_t_K4A_CALIBRATION_LENS_DISTORTION_MODEL_THETA,
    #[doc = "< Deprecated (not supported). Calibration model is Polynomial 3K."]
    Polynomial3K = k4a_calibration_model_type_t_K4A_CALIBRATION_LENS_DISTORTION_MODEL_POLYNOMIAL_3K,
    #[doc = "< Deprecated (only supported early internal devices). Calibration model is Rational 6KT."]
    Rational6KT = k4a_calibration_model_type_t_K4A_CALIBRATION_LENS_DISTORTION_MODEL_RATIONAL_6KT,
    #[doc = "< Calibration model is Brown Conrady (compatible with OpenCV)"]
    BrownConrady = k4a_calibration_model_type_t_K4A_CALIBRATION_LENS_DISTORTION_MODEL_BROWN_CONRADY,
}

impl_conv_primitive_to_enum!(CalibrationModelType, k4a_calibration_model_type_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Firmware build type."]
pub enum FirmwareBuildType {
    #[doc = "< Production firmware."]
    Release = k4a_firmware_build_t_K4A_FIRMWARE_BUILD_RELEASE,
    #[doc = "< Pre-production firmware."]
    Debug = k4a_firmware_build_t_K4A_FIRMWARE_BUILD_DEBUG,
}

impl_conv_primitive_to_enum!(FirmwareBuildType, k4a_firmware_build_t);

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[doc = " Firmware signature type."]
pub enum FirmwareSignatureType {
    #[doc = "< Microsoft signed firmware."]
    Microsoft = k4a_firmware_signature_t_K4A_FIRMWARE_SIGNATURE_MSFT,
    #[doc = "< Test signed firmware."]
    Test = k4a_firmware_signature_t_K4A_FIRMWARE_SIGNATURE_TEST,
    #[doc = "< Unsigned firmware."]
    Unsigned = k4a_firmware_signature_t_K4A_FIRMWARE_SIGNATURE_UNSIGNED,
}

impl_conv_primitive_to_enum!(FirmwareSignatureType, k4a_firmware_signature_t);
