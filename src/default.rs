use super::bindings::*;

impl Default for k4a_color_control_mode_t {
    fn default() -> Self { k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_AUTO }
}

impl Default for k4a_device_configuration_t {
    fn default() -> Self {
        k4a_device_configuration_t {
            color_format: k4a_image_format_t::K4A_IMAGE_FORMAT_COLOR_BGRA32,
            color_resolution: k4a_color_resolution_t::K4A_COLOR_RESOLUTION_720P,
            depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_2X2BINNED,
            camera_fps: k4a_fps_t::K4A_FRAMES_PER_SECOND_30,
            synchronized_images_only: false,
            depth_delay_off_color_usec: 0,
            wired_sync_mode: k4a_wired_sync_mode_t::K4A_WIRED_SYNC_MODE_STANDALONE,
            subordinate_delay_off_master_usec: 0,
            disable_streaming_indicator: false,
        }
    }
}

impl Default for k4a_float2_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_float3_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_imu_sample_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_calibration_extrinsics_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_calibration_intrinsics_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_calibration_camera_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_calibration_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for k4a_hardware_version_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

