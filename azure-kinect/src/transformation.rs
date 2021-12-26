#![allow(non_upper_case_globals)]

use crate::enums::CalibrationType;
use crate::*;
use azure_kinect_sys::k4a::*;
use std::ptr;

#[allow(dead_code)]
pub struct Transformation<'a> {
    factory: &'a Factory,
    handle: k4a_transformation_t,
    color_resolution: Dimension,
    depth_resolution: Dimension,
}

impl<'a> Transformation<'a> {
    #[deprecated(since = "0.2", note = "Factory::transformation_create")]
    pub fn new(factory: &'a Factory, calibration: &'a Calibration) -> Transformation<'a> {
        factory.transformation_create(calibration)
    }

    pub fn from_handle(
        factory: &'a Factory,
        handle: k4a_transformation_t,
        calibration: &'a Calibration,
    ) -> Transformation<'a> {
        Transformation {
            factory,
            handle,
            color_resolution: Dimension {
                width: calibration
                    .calibration
                    .color_camera_calibration
                    .resolution_width,
                height: calibration
                    .calibration
                    .color_camera_calibration
                    .resolution_height,
            },
            depth_resolution: Dimension {
                width: calibration
                    .calibration
                    .depth_camera_calibration
                    .resolution_width,
                height: calibration
                    .calibration
                    .depth_camera_calibration
                    .resolution_height,
            },
        }
    }

    pub fn depth_image_to_color_camera_exist_image(
        &self,
        depth_image: &Image,
        transformed_depth_image: &mut Image,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api
                .funcs
                .k4a_transformation_depth_image_to_color_camera)(
                self.handle,
                depth_image.handle,
                transformed_depth_image.handle,
            )
        })
        .to_result(())
    }

    pub fn depth_image_to_color_camera(&self, depth_image: &Image) -> Result<Image, Error> {
        let mut transformed_depth_image = self.factory.image_create(
            ImageFormat::Depth16,
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (std::mem::size_of::<u16>() as i32),
        )?;
        self.depth_image_to_color_camera_exist_image(depth_image, &mut transformed_depth_image)?;
        Ok(transformed_depth_image)
    }

    pub fn depth_image_to_color_camera_custom_exist_image(
        &self,
        depth_image: &Image,
        custom_image: &Image,
        transformed_depth_image: &mut Image,
        transformed_custom_image: &mut Image,
        interpolation_type: TransformationInterpolationType,
        invalid_custom_value: u32,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api
                .funcs
                .k4a_transformation_depth_image_to_color_camera_custom)(
                self.handle,
                depth_image.handle,
                custom_image.handle,
                transformed_depth_image.handle,
                transformed_custom_image.handle,
                interpolation_type.into(),
                invalid_custom_value,
            )
        })
        .to_result(())
    }

    pub fn depth_image_to_color_camera_custom(
        &self,
        depth_image: &Image,
        custom_image: &Image,
        interpolation_type: TransformationInterpolationType,
        invalid_custom_value: u32,
    ) -> Result<(Image, Image), Error> {
        let bytes_per_pixel: usize = match custom_image.get_format() {
            ImageFormat::Custom8 => std::mem::size_of::<i8>(),
            ImageFormat::Custom16 => std::mem::size_of::<i16>(),
            _ => return Err(Error::Failed),
        };

        let mut transformed_depth_image = self.factory.image_create(
            ImageFormat::Depth16,
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (std::mem::size_of::<u16>() as i32),
        )?;

        let mut transformed_custom_image = self.factory.image_create(
            custom_image.get_format(),
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (bytes_per_pixel as i32),
        )?;

        self.depth_image_to_color_camera_custom_exist_image(
            depth_image,
            custom_image,
            &mut transformed_depth_image,
            &mut transformed_custom_image,
            interpolation_type.into(),
            invalid_custom_value,
        )?;
        Ok((transformed_depth_image, transformed_custom_image))
    }

    pub fn color_image_to_depth_camera_exist_image(
        &self,
        depth_image: &Image,
        color_image: &Image,
        transformed_color_image: &mut Image,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api
                .funcs
                .k4a_transformation_color_image_to_depth_camera)(
                self.handle,
                depth_image.handle,
                color_image.handle,
                transformed_color_image.handle,
            )
        })
        .to_result(())
    }

    pub fn color_image_to_depth_camera(
        &self,
        depth_image: &Image,
        color_image: &Image,
    ) -> Result<Image, Error> {
        let mut transformed_color_image = self.factory.image_create(
            ImageFormat::BGRA32,
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (std::mem::size_of::<u8>() as i32) * 4,
        )?;

        self.color_image_to_depth_camera_exist_image(
            depth_image,
            color_image,
            &mut transformed_color_image,
        )?;
        Ok(transformed_color_image)
    }

    pub fn depth_image_to_point_cloud_exist_image(
        &self,
        depth_image: &Image,
        camera: CalibrationType,
        xyz_image: &mut Image,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api
                .funcs
                .k4a_transformation_depth_image_to_point_cloud)(
                self.handle,
                depth_image.handle,
                camera.into(),
                xyz_image.handle,
            )
        })
        .to_result(())
    }

    pub fn depth_image_to_point_cloud(
        &self,
        depth_image: &Image,
        camera: CalibrationType,
    ) -> Result<Image, Error> {
        let mut xyz_image = self.factory.image_create(
            ImageFormat::Custom,
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (std::mem::size_of::<u16>() as i32) * 3,
        )?;
        self.depth_image_to_point_cloud_exist_image(depth_image, camera, &mut xyz_image)?;
        Ok(xyz_image)
    }
}

impl Drop for Transformation<'_> {
    fn drop(&mut self) {
        unsafe {
            (self.factory.api.funcs.k4a_transformation_destroy)(self.handle);
        }
        self.handle = ptr::null_mut();
    }
}
