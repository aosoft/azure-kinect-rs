use super::bindings::*;
use std::default::Default;

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
