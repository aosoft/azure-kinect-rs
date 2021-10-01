use super::*;

pub struct Calibration<'a> {
    api: &'a Api,
    pub(crate) calibration: k4a_calibration_t,
}

impl Calibration<'_> {
    pub(crate) fn from_handle(api: &Api, calibration: k4a_calibration_t) -> Calibration {
        Calibration {
            api: api,
            calibration: calibration,
        }
    }

    pub fn from_raw<'a>(
        api: &'a Api,
        raw_calibration: &Vec<u8>,
        target_depth_mode: k4a_depth_mode_t,
        target_color_resolution: k4a_color_resolution_t,
    ) -> Result<Calibration<'a>, Error> {
        let mut calibration = k4a_calibration_t::default();
        Error::from((api.k4a_calibration_get_from_raw)(
            raw_calibration.as_ptr() as *mut i8,
            raw_calibration.len(),
            target_depth_mode,
            target_color_resolution,
            &mut calibration,
        ))
        .to_result_fn(|| Calibration::from_handle(api, calibration))
    }

    /// Transform a 3d point of a source coordinate system into a 3d point of the target coordinate system.
    pub fn convert_3d_to_3d(
        &self,
        source_point3d: &k4a_float3_t,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<k4a_float3_t, Error> {
        let mut target_point3d = k4a_float3_t::default();
        Error::from((self.api.k4a_calibration_3d_to_3d)(
            &self.calibration,
            source_point3d,
            source_camera,
            target_camera,
            &mut target_point3d,
        ))
        .to_result(target_point3d)
    }

    /// Transform a 2d pixel coordinate with an associated depth value of the source camera into a 3d point of the target coordinate system.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point3d should not be used)
    pub fn convert_2d_to_3d(
        &self,
        source_point2d: &k4a_float2_t,
        source_depth: f32,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float3_t, bool), Error> {
        let mut target_point3d = k4a_float3_t::default();
        let mut valid: i32 = 0;
        Error::from((self.api.k4a_calibration_2d_to_3d)(
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

    /// Transform a 3d point of a source coordinate system into a 2d pixel coordinate of the target camera.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_3d_to_2d(
        &self,
        source_point3d: &k4a_float3_t,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float2_t, bool), Error> {
        let mut target_point2d = k4a_float2_t::default();
        let mut valid: i32 = 0;
        Error::from((self.api.k4a_calibration_3d_to_2d)(
            &self.calibration,
            source_point3d,
            source_camera,
            target_camera,
            &mut target_point2d,
            &mut valid,
        ))
        .to_result((target_point2d, valid != 0))
    }

    /// Transform a 2d pixel coordinate with an associated depth value of the source camera into a 2d pixel coordinate of the target camera
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_2d_to_2d(
        &self,
        source_point2d: &k4a_float2_t,
        source_depth: f32,
        source_camera: k4a_calibration_type_t,
        target_camera: k4a_calibration_type_t,
    ) -> Result<(k4a_float2_t, bool), Error> {
        let mut target_point2d = k4a_float2_t::default();
        let mut valid: i32 = 0;
        Error::from((self.api.k4a_calibration_2d_to_2d)(
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

    /// Transform a 2D pixel coordinate from color camera into a 2D pixel coordinate of the depth camera. This function
    /// searches along an epipolar line in the depth image to find the corresponding depth pixel.
    /// Returns false if the point is invalid in the target coordinate system (and therefore target_point2d should not be used)
    pub fn convert_color_2d_to_depth_2d(
        &self,
        source_point2d: &k4a_float2_t,
        depth_image: &Image,
    ) -> Result<(k4a_float2_t, bool), Error> {
        let mut target_point2d = k4a_float2_t::default();
        let mut valid: i32 = 0;
        Error::from((self.api.k4a_calibration_color_2d_to_depth_2d)(
            &self.calibration,
            source_point2d,
            depth_image.handle,
            &mut target_point2d,
            &mut valid,
        ))
        .to_result((target_point2d, valid != 0))
    }
}
