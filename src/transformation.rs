use super::bindings::*;
use super::calibration::Calibration;
use super::error::Error;
use super::factory::Factory;
use super::image::Image;
use std::ptr;

pub struct Resolution {
    width: i32,
    height: i32,
}

pub struct Transformation<'a> {
    factory: &'a Factory,
    handle: k4a_transformation_t,
    color_resolution: Resolution,
    depth_resolution: Resolution,
}

impl Transformation<'_> {
    pub fn new<'a>(factory: &'a Factory, calibration: &'a Calibration) -> Transformation<'a> {
        let handle = (factory.k4a_transformation_create)(&calibration.calibration);
        Transformation {
            factory: factory,
            handle: handle,
            color_resolution: Resolution {
                width: calibration
                    .calibration
                    .color_camera_calibration
                    .resolution_width,
                height: calibration
                    .calibration
                    .color_camera_calibration
                    .resolution_height,
            },
            depth_resolution: Resolution {
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
}

impl Drop for Transformation<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_transformation_destroy)(self.handle);
        self.handle = ptr::null_mut();
    }
}
