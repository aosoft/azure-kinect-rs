use super::bindings::*;

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

/*
impl Default for k4a_float2_t__xy {
    fn default() -> Self {
        k4a_float2_t__xy { x: 0.0, y: 0.0 }
    }
}

impl Default for k4a_float2_t {
    fn default() -> Self {
        k4a_float2_t {
            xy: k4a_float2_t__xy::default(),
        }
    }
}

impl Default for k4a_float3_t__xyz {
    fn default() -> Self {
        k4a_float3_t__xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Default for k4a_float3_t {
    fn default() -> Self {
        k4a_float3_t {
            xyz: k4a_float3_t__xyz::default(),
        }
    }
}

impl Default for k4a_imu_sample_t {
    fn default() -> Self {
        k4a_imu_sample_t {
            acc_sample: k4a_float3_t::default(),
            acc_timestamp_usec: 0,
            gyro_sample: k4a_float3_t::default(),
            gyro_timestamp_usec: 0,
            temperature: 0.0,
        }
    }
}

impl Default for k4a_calibration_extrinsics_t {
    fn default() -> Self {
        k4a_calibration_extrinsics_t {
            rotation: [0.0; 9],
            translation: [0.0; 3],
        }
    }
}

impl Default for k4a_calibration_intrinsic_parameters_t {
    fn default() -> Self {
        k4a_calibration_intrinsic_parameters_t {
            v: [0.0; 15]
        }
    }
}

impl Default for k4a_calibration_intrinsics_t {
    fn default() -> Self {
        k4a_calibration_intrinsics_t {
            type_: k4a_calibration_model_type_t::K4A_CALIBRATION_LENS_DISTORTION_MODEL_UNKNOWN,
            parameter_count: 0,
            parameters: k4a_calibration_intrinsic_parameters_t::default(),
        }
    }
}

impl Default for k4a_calibration_camera_t {
    fn default() -> Self {
        k4a_calibration_camera_t {
            extrinsics: k4a_calibration_extrinsics_t::default(),
            intrinsics: k4a_calibration_intrinsics_t::default(),
            resolution_width: 0,
            resolution_height: 0,
            metric_radius: 0.0,
        }
    }
}

impl Default for k4a_calibration_t {
    fn default() -> Self {
        k4a_calibration_t {
            depth_camera_calibration:k4a_calibration_camera_t::default(),
            color_camera_calibration:k4a_calibration_camera_t::default(),
            extrinsics: [[k4a_calibration_extrinsics_t::default(); 4]; 4],
            depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_OFF,
            color_resolution: k4a_color_resolution_t::K4A_COLOR_RESOLUTION_OFF
        }
    }
}
*/
