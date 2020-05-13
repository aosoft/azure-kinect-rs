use azure_kinect::*;

#[test]
fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let factory = factory::Factory::load(
        std::env::current_dir()?
            .to_str()
            .ok_or(error::Error::Failed)?,
    )?
    .set_debug_message_handler(
        Box::new(|level, file, line, message| {
            println!("{:?}, {}, {}, {}", level, file, line, message);
        }),
        k4a_log_level_t::K4A_LOG_LEVEL_ERROR,
    );

    let c = factory.device_get_installed_count();
    println!("device count = {}", c);
    let device = factory.device_open(0)?;
    let serial = device.get_serialnum()?;
    let version = device.get_version()?;
    println!("serial = {} / hw ver = {:?}", serial, version);

    let color_control =
        device.get_color_control(k4a_color_control_command_t::K4A_COLOR_CONTROL_BRIGHTNESS)?;
    println!("color control(brightness) = {:?}", color_control);

    let camera_config = k4a_device_configuration_t::default();
    let camera = device.start_cameras(&camera_config)?;

    {
        let imu = camera.start_imu()?;
        let imu_sample = imu.get_imu_sample_wait_infinite()?;
        println!("imu = {}", imu_sample);
    }

    for i in 0..20 {
        if let Ok(capture) = camera.get_capture(100) {
            let image = capture.get_color_image();
            println!(
                "[{}] format = {:?}, width = {}, height = {}, temparature = {}",
                i,
                image.get_format(),
                image.get_width_pixels(),
                image.get_height_pixels(),
                capture.get_temperature_c()
            );
        }
    }

    Ok(())
}
