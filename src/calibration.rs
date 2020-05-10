use super::bindings::*;
use super::error::Error;
use super::factory::Factory;
use super::image::Image;
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

    pub fn convert_2d_to_3d(
        &self,
        source_point2d: &k4a_float2_t,
        source_depth: f32,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float3_t, bool), Error> {
        unsafe {
            let mut target_point3d = k4a_float3_t::default();
            let mut valid: i32 = 0;
            Error::from((self.factory.k4a_calibration_2d_to_3d)(
                &self.calibration,
                source_point2d,
                source_depth,
                source_camera,
                target_camera,
                &mut target_point3d,
                &mut valid,
            ))
            .to_result((target_point3d, valid != 0))
        }
    }

    pub fn convert_3d_to_2d(
        &self,
        source_point3d: &k4a_float3_t,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float2_t, bool), Error> {
        unsafe {
            let mut target_point2d = k4a_float2_t::default();
            let mut valid: i32 = 0;
            Error::from((self.factory.k4a_calibration_3d_to_2d)(
                &self.calibration,
                source_point3d,
                source_camera,
                target_camera,
                &mut target_point2d,
                &mut valid,
            ))
            .to_result((target_point2d, valid != 0))
        }
    }

    pub fn convert_2d_to_2d(
        &self,
        source_point2d: &k4a_float2_t,
        source_depth: f32,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float2_t, bool), Error> {
        unsafe {
            let mut target_point2d = k4a_float2_t::default();
            let mut valid: i32 = 0;
            Error::from((self.factory.k4a_calibration_2d_to_2d)(
                &self.calibration,
                source_point2d,
                source_depth,
                source_camera,
                target_camera,
                &mut target_point2d,
                &mut valid,
            ))
            .to_result((target_point2d, valid != 0))
        }
    }

    pub fn convert_color_2d_to_depth_2d(
        &self,
        source_point2d: &k4a_float2_t,
        depth_image: &Image,
    ) -> Result<(k4a_float2_t, bool), Error> {
        unsafe {
            let mut target_point2d = k4a_float2_t::default();
            let mut valid: i32 = 0;
            Error::from((self.factory.k4a_calibration_color_2d_to_depth_2d)(
                &self.calibration,
                source_point2d,
                depth_image.handle,
                &mut target_point2d,
                &mut valid,
            ))
            .to_result((target_point2d, valid != 0))
        }
    }
}
