
macro_rules! k4a_declare_handle {
    ($name:ident, $sname:ident) => {
        #[allow(non_camel_case_types)]
        pub(crate) struct $sname {
            rsvd: isize
        }

        #[allow(non_camel_case_types)]
        pub(crate) type $name = *mut $sname;
    }
}

k4a_declare_handle!(k4a_device_t, _k4a_device_t);
k4a_declare_handle!(k4a_capture_t, _k4a_capture_t);
k4a_declare_handle!(k4a_image_t, _k4a_image_t);
k4a_declare_handle!(k4a_transformation_t, _k4a_transformation_t);

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_result_t {
    K4A_RESULT_SUCCEEDED = 0,
    K4A_RESULT_FAILED
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_buffer_result_t {
    K4A_BUFFER_RESULT_SUCCEEDED = 0,
    K4A_BUFFER_RESULT_FAILED,
    K4A_BUFFER_RESULT_TOO_SMALL,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_wait_result_t
{
    K4A_WAIT_RESULT_SUCCEEDED = 0,
    K4A_WAIT_RESULT_FAILED,
    K4A_WAIT_RESULT_TIMEOUT,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_log_level_t {
    K4A_LOG_LEVEL_CRITICAL = 0,
    K4A_LOG_LEVEL_ERROR,
    K4A_LOG_LEVEL_WARNING,
    K4A_LOG_LEVEL_INFO,
    K4A_LOG_LEVEL_TRACE,
    K4A_LOG_LEVEL_OFF,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_depth_mode_t {
    K4A_DEPTH_MODE_OFF = 0,
    K4A_DEPTH_MODE_NFOV_2X2BINNED,
    K4A_DEPTH_MODE_NFOV_UNBINNED,
    K4A_DEPTH_MODE_WFOV_2X2BINNED,
    K4A_DEPTH_MODE_WFOV_UNBINNED,
    K4A_DEPTH_MODE_PASSIVE_IR,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_color_resolution_t {
    K4A_COLOR_RESOLUTION_OFF = 0,
    K4A_COLOR_RESOLUTION_720P,
    K4A_COLOR_RESOLUTION_1080P,
    K4A_COLOR_RESOLUTION_1440P,
    K4A_COLOR_RESOLUTION_1536P,
    K4A_COLOR_RESOLUTION_2160P,
    K4A_COLOR_RESOLUTION_3072P,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_image_format_t {
    K4A_IMAGE_FORMAT_COLOR_MJPG = 0,
    K4A_IMAGE_FORMAT_COLOR_NV12,
    K4A_IMAGE_FORMAT_COLOR_YUY2,
    K4A_IMAGE_FORMAT_COLOR_BGRA32,
    K4A_IMAGE_FORMAT_DEPTH16,
    K4A_IMAGE_FORMAT_IR16,
    K4A_IMAGE_FORMAT_CUSTOM8,
    K4A_IMAGE_FORMAT_CUSTOM16,
    K4A_IMAGE_FORMAT_CUSTOM,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_transformation_interpolation_type_t {
    K4A_TRANSFORMATION_INTERPOLATION_TYPE_NEAREST = 0,
    K4A_TRANSFORMATION_INTERPOLATION_TYPE_LINEAR,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_fps_t {
    K4A_FRAMES_PER_SECOND_5 = 0,
    K4A_FRAMES_PER_SECOND_15,
    K4A_FRAMES_PER_SECOND_30,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_color_control_command_t {
    K4A_COLOR_CONTROL_EXPOSURE_TIME_ABSOLUTE = 0,
    K4A_COLOR_CONTROL_AUTO_EXPOSURE_PRIORITY,
    K4A_COLOR_CONTROL_BRIGHTNESS,
    K4A_COLOR_CONTROL_CONTRAST,
    K4A_COLOR_CONTROL_SATURATION,
    K4A_COLOR_CONTROL_SHARPNESS,
    K4A_COLOR_CONTROL_WHITEBALANCE,
    K4A_COLOR_CONTROL_BACKLIGHT_COMPENSATION,
    K4A_COLOR_CONTROL_GAIN,
    K4A_COLOR_CONTROL_POWERLINE_FREQUENCY
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_color_control_mode_t {
    K4A_COLOR_CONTROL_MODE_AUTO = 0,
    K4A_COLOR_CONTROL_MODE_MANUAL,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_wired_sync_mode_t {
    K4A_WIRED_SYNC_MODE_STANDALONE,
    K4A_WIRED_SYNC_MODE_MASTER,
    K4A_WIRED_SYNC_MODE_SUBORDINATE
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_calibration_type_t {
    K4A_CALIBRATION_TYPE_UNKNOWN = -1,
    K4A_CALIBRATION_TYPE_DEPTH,
    K4A_CALIBRATION_TYPE_COLOR,
    K4A_CALIBRATION_TYPE_GYRO,
    K4A_CALIBRATION_TYPE_ACCEL,
    K4A_CALIBRATION_TYPE_NUM,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_calibration_model_type_t {
    K4A_CALIBRATION_LENS_DISTORTION_MODEL_UNKNOWN = 0,
    K4A_CALIBRATION_LENS_DISTORTION_MODEL_THETA,
    K4A_CALIBRATION_LENS_DISTORTION_MODEL_POLYNOMIAL_3K,
    K4A_CALIBRATION_LENS_DISTORTION_MODEL_RATIONAL_6KT,
    K4A_CALIBRATION_LENS_DISTORTION_MODEL_BROWN_CONRADY,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum k4a_firmware_build_t {
    K4A_FIRMWARE_BUILD_RELEASE,
    K4A_FIRMWARE_BUILD_DEBUG
}

#[allow(non_camel_case_types)]
#[repr(C)]
enum k4a_firmware_signature_t {
    K4A_FIRMWARE_SIGNATURE_MSFT,
    K4A_FIRMWARE_SIGNATURE_TEST,
    K4A_FIRMWARE_SIGNATURE_UNSIGNED
}

pub(crate) type k4a_logging_message_cb_t = fn(context: *mut (),
                                              level: k4a_log_level_t,
                                              file: &str,
                                              line: i32,
                                              message: &str);
pub(crate) type k4a_memory_destroy_cb_t = fn(buffer: *mut (),
                                             context: *mut ());
pub(crate) type k4a_memory_allocate_cb_t = fn(size: i32,
                                              context: *mut *mut ());

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_device_configuration_t {
    color_format: k4a_image_format_t,
    color_resolution: k4a_color_resolution_t,
    depth_mode: k4a_depth_mode_t,
    camera_fps: k4a_fps_t,
    synchronized_images_only: bool,
    depth_delay_off_color_usec: i32,
    wired_sync_mode: k4a_wired_sync_mode_t,
    subordinate_delay_off_master_usec: u32,
    disable_streaming_indicator: bool
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_calibration_extrinsics_t {
    rotation: [f32; 9],
    translation: [f32; 3]
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct k4a_calibration_intrinsic_parameters_param {
    cx: f32,
    cy: f32,
    fx: f32,
    fy: f32,
    k1: f32,
    k2: f32,
    k3: f32,
    k4: f32,
    k5: f32,
    k6: f32,
    codx: f32,
    cody: f32,
    p2: f32,
    p1: f32,
    metric_radius: f32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) union k4a_calibration_intrinsic_parameters_t {
    param: k4a_calibration_intrinsic_parameters_param,
    v: [f32; 15]
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_calibration_intrinsics_t {
    type_: k4a_calibration_model_type_t,
    parameter_count: u32,
    parameters: k4a_calibration_intrinsic_parameters_t
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_calibration_camera_t {
    extrinsics: k4a_calibration_extrinsics_t,
    intrinsics: k4a_calibration_intrinsics_t,
    resolution_width: i32,
    resolution_height: i32,
    metric_radius: f32
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_calibration_t {
    depth_camera_calibration: k4a_calibration_camera_t,
    color_camera_calibration: k4a_calibration_camera_t,
    extrinsics: [[k4a_calibration_extrinsics_t; k4a_calibration_type_t::K4A_CALIBRATION_TYPE_NUM as usize]; k4a_calibration_type_t::K4A_CALIBRATION_TYPE_NUM as usize],
    depth_mode: k4a_depth_mode_t,
    color_resolution: k4a_color_resolution_t
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_version_t {
    major: u32,
    minor: u32,
    iteration: u32
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_hardware_version_t {
    rgb: k4a_version_t,
    depth: k4a_version_t,
    audio: k4a_version_t,
    depth_sensor: k4a_version_t,
    firmware_build: k4a_firmware_build_t,
    firmware_signature: k4a_firmware_signature_t
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct k4a_float2_xy {
    x: f32,
    y: f32
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) union k4a_float2_t {
    xy: k4a_float2_xy,
    v: [f32; 2]
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct k4a_float3_xyz {
    x: f32,
    y: f32,
    z: f32
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) union k4a_float3_t {
    xyz: k4a_float3_xyz,
    v: [f32; 3]
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct k4a_imu_sample_t {
    temperature: f32,
    acc_sample: k4a_float3_t,
    acc_timestamp_usec: u64,
    gyro_sample: k4a_float3_t,
    gyro_timestamp_usec: u64
}

pub(crate) const K4A_DEVICE_DEFAULT: i32 = 0;
pub(crate) const K4A_WAIT_INFINITE: i32 = -1;

pub(crate) const K4A_DEVICE_CONFIG_INIT_DISABLE_ALL: k4a_device_configuration_t  = k4a_device_configuration_t {
    color_format: k4a_image_format_t::K4A_IMAGE_FORMAT_COLOR_MJPG,
    color_resolution: k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF,
    depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_OFF,
    camera_fps: k4a_fps_t::K4A_FRAMES_PER_SECOND_30,
    synchronized_images_only: false,
    depth_delay_off_color_usec: 0,
    wired_sync_mode: k4a_wired_sync_mode_t::K4A_WIRED_SYNC_MODE_STANDALONE,
    subordinate_delay_off_master_usec: 0,
    disable_streaming_indicator: false
};


