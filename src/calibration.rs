use super::bindings::*;
use super::error::Error;
use super::factory::Factory;
use std::ptr;

pub struct Calibration<'a> {
    factory: &'a Factory,
    calibration: k4a_calibration_t,
}

impl Calibration<'_> {
    pub(crate) fn new(factory: &Factory, calibration: k4a_calibration_t) -> Calibration {
        Calibration {
            factory: factory,
            calibration: calibration,
        }
    }

    pub fn convert_3d_to_3d(
        &self,
        source_point3d: &k4a_float3_t,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<k4a_float3_t, Error> {
        unsafe {
            let mut target_point3d = k4a_float3_t::default();
            Error::from((self.factory.k4a_calibration_3d_to_3d)(
                &self.calibration,
                source_point3d,
                source_camera,
                target_camera,
                &mut target_point3d,
            ))
            .to_result(target_point3d)
        }
    }
}
