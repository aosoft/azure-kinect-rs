use super::*;
use std::fmt::{Display, Formatter, Result};

impl Display for k4a_imu_sample_t {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            write!(f,
                   "temperature: {}, acc_sample: {:?}, acc_timestamp_usec: {}, gyro_sample: {:?} gyro_timestamp_usec: {}",
                   self.temperature,
                   self.acc_sample.xyz,
                   self.acc_timestamp_usec,
                   self.gyro_sample.xyz,
                   self.gyro_timestamp_usec)
        }
    }
}
