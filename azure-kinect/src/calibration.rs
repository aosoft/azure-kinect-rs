use crate::*;
use azure_kinect_sys::k4a::*;
use crate::enums::CalibrationType;

pub struct Calibration<'a> {
    api: &'a azure_kinect_sys::api::Api,
    pub(crate) calibration: k4a_calibration_t,
}

impl Calibration<'_> {
    pub(crate) fn from_handle(
        api: &azure_kinect_sys::api::Api,
        calibration: k4a_calibration_t,
    ) -> Calibration {
        Calibration {
            api,
            calibration,
        }
    }

    pub fn from_raw<'a>(
        factory: &'a Factory,
        raw_calibration: &Vec<u8>,
        target_depth_mode: DepthMode,
        target_color_resolution: ColorResolution,
    ) -> Result<Calibration<'a>, Error> {
        let mut calibration = k4a_calibration_t::default();
        Error::from_k4a_result_t(unsafe {
            (factory.api.funcs.k4a_calibration_get_from_raw)(
                raw_calibration.as_ptr() as *mut i8,
                raw_calibration.len(),
                target_depth_mode.into(),
                target_color_resolution.into(),
                &mut calibration,
            )
        })
        .to_result_fn(|| Calibration::from_handle(&factory.api, calibration))
    }

    /// Transform a 3d point of a source coordinate system into a 3d point of the target coordinate system.
    pub fn convert_3d_to_3d(
        &self,
        source_point3d: &Float3,
        source_camera: CalibrationType,
        target_camera: CalibrationType,
    ) -> Result<Float3, Error> {
        let mut target_point3d = Float3::default();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_3d_to_3d)(
                &self.calibration,
                &source_point3d.value,
                source_camera.into(),
                target_camera.into(),
                &mut target_point3d.value,
            )
        })
        .to_result(target_point3d)
    }

    /// Transform a 2d pixel coordinate with an associated depth value of the source camera into a 3d point of the target coordinate system.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point3d should not be used)
    pub fn convert_2d_to_3d(
        &self,
        source_point2d: &Float2,
        source_depth: f32,
        source_camera: CalibrationType,
        target_camera: CalibrationType,
    ) -> Result<(Float3, bool), Error> {
        let mut target_point3d = Float3::default();
        let mut valid: i32 = 0;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_2d_to_3d)(
                &self.calibration,
                &source_point2d.value,
                source_depth,
                source_camera.into(),
                target_camera.into(),
                &mut target_point3d.value,
                &mut valid,
            )
        })
        .to_result((target_point3d, valid != 0))
    }

    /// Transform a 3d point of a source coordinate system into a 2d pixel coordinate of the target camera.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_3d_to_2d(
        &self,
        source_point3d: &Float3,
        source_camera: CalibrationType,
        target_camera: CalibrationType,
    ) -> Result<(Float2, bool), Error> {
        let mut target_point2d = Float2::default();
        let mut valid: i32 = 0;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_3d_to_2d)(
                &self.calibration,
                &source_point3d.value,
                source_camera.into(),
                target_camera.into(),
                &mut target_point2d.value,
                &mut valid,
            )
        })
        .to_result((target_point2d, valid != 0))
    }

    /// Transform a 2d pixel coordinate with an associated depth value of the source camera into a 2d pixel coordinate of the target camera
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_2d_to_2d(
        &self,
        source_point2d: &Float2,
        source_depth: f32,
        source_camera: CalibrationType,
        target_camera: CalibrationType,
    ) -> Result<(Float2, bool), Error> {
        let mut target_point2d = Float2::default();
        let mut valid: i32 = 0;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_2d_to_2d)(
                &self.calibration,
                &source_point2d.value,
                source_depth,
                source_camera.into(),
                target_camera.into(),
                &mut target_point2d.value,
                &mut valid,
            )
        })
        .to_result((target_point2d, valid != 0))
    }

    /// Transform a 2D pixel coordinate from color camera into a 2D pixel coordinate of the depth camera. This function
    /// searches along an epipolar line in the depth image to find the corresponding depth pixel.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_color_2d_to_depth_2d(
        &self,
        source_point2d: &Float2,
        depth_image: &Image,
    ) -> Result<(Float2, bool), Error> {
        let mut target_point2d = Float2::default();
        let mut valid: i32 = 0;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_color_2d_to_depth_2d)(
                &self.calibration,
                &source_point2d.value,
                depth_image.handle,
                &mut target_point2d.value,
                &mut valid,
            )
        })
        .to_result((target_point2d, valid != 0))
    }
}
