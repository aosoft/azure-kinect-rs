use azure_kinect::*;

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let factory = Factory::new()?;
    let device = factory.device_open(0)?;
    let camera_config = k4a_device_configuration_t::default();
    let camera = device.start_cameras(&camera_config)?;

    if let Ok(capture) = camera.get_capture(1000) {
        let image = capture.get_color_image();
        println!(
            "format = {:?}, width = {}, height = {}, temparature = {}",
            image.get_format(),
            image.get_width_pixels(),
            image.get_height_pixels(),
            capture.get_temperature_c()
        );
    }

    Ok(())
}
