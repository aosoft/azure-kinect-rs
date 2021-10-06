use crate::*;
use std::ptr;

#[allow(dead_code)]
pub struct Transformation<'a> {
    factory: &'a Factory,
    handle: k4a_transformation_t,
    color_resolution: Dimension,
    depth_resolution: Dimension,
}

impl Transformation<'_> {
    pub fn new<'a>(factory: &'a Factory, calibration: &'a Calibration) -> Transformation<'a> {
        let handle = (factory.k4a_transformation_create)(&calibration.calibration);
        Transformation {
            factory: factory,
            handle: handle,
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
        Error::from((self
            .factory
            .k4a_transformation_depth_image_to_color_camera)(
            self.handle,
            depth_image.handle,
            transformed_depth_image.handle,
        ))
        .to_result(())
    }

    pub fn depth_image_to_color_camera(&self, depth_image: &Image) -> Result<Image, Error> {
        let mut transformed_depth_image = Image::with_format(
            self.factory,
            k4a_image_format_t::K4A_IMAGE_FORMAT_DEPTH16,
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
        interpolation_type: k4a_transformation_interpolation_type_t,
        invalid_custom_value: u32,
    ) -> Result<(), Error> {
        Error::from((self
            .factory
            .k4a_transformation_depth_image_to_color_camera_custom)(
            self.handle,
            depth_image.handle,
            custom_image.handle,
            transformed_depth_image.handle,
            transformed_custom_image.handle,
            interpolation_type,
            invalid_custom_value,
        ))
        .to_result(())
    }

    pub fn depth_image_to_color_camera_custom(
        &self,
        depth_image: &Image,
        custom_image: &Image,
        interpolation_type: k4a_transformation_interpolation_type_t,
        invalid_custom_value: u32,
    ) -> Result<(Image, Image), Error> {
        let bytes_per_pixel: usize = match custom_image.get_format() {
            k4a_image_format_t::K4A_IMAGE_FORMAT_CUSTOM8 => std::mem::size_of::<i8>(),
            k4a_image_format_t::K4A_IMAGE_FORMAT_CUSTOM16 => std::mem::size_of::<i16>(),
            _ => return Err(Error::Failed),
        };

        let mut transformed_depth_image = Image::with_format(
            self.factory,
            k4a_image_format_t::K4A_IMAGE_FORMAT_DEPTH16,
            self.color_resolution.width,
            self.color_resolution.height,
            self.color_resolution.width * (std::mem::size_of::<u16>() as i32),
        )?;

        let mut transformed_custom_image = Image::with_format(
            self.factory,
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
            interpolation_type,
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
        Error::from((self
            .factory
            .k4a_transformation_color_image_to_depth_camera)(
            self.handle,
            depth_image.handle,
            color_image.handle,
            transformed_color_image.handle,
        ))
        .to_result(())
    }

    pub fn color_image_to_depth_camera(
        &self,
        depth_image: &Image,
        color_image: &Image,
    ) -> Result<Image, Error> {
        let mut transformed_color_image = Image::with_format(
            self.factory,
            k4a_image_format_t::K4A_IMAGE_FORMAT_COLOR_BGRA32,
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
        camera: k4a_calibration_type_t,
        xyz_image: &mut Image,
    ) -> Result<(), Error> {
        Error::from(
            (self.factory.k4a_transformation_depth_image_to_point_cloud)(
                self.handle,
                depth_image.handle,
                camera,
                xyz_image.handle,
            ),
        )
        .to_result(())
    }

    pub fn depth_image_to_point_cloud(
        &self,
        depth_image: &Image,
        camera: k4a_calibration_type_t,
    ) -> Result<Image, Error> {
        let mut xyz_image = Image::with_format(
            self.factory,
            k4a_image_format_t::K4A_IMAGE_FORMAT_CUSTOM,
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
        (self.factory.k4a_transformation_destroy)(self.handle);
        self.handle = ptr::null_mut();
    }
}
