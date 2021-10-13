use azure_kinect::*;

//  Copy depthengine_2_0.dll and k4a.dll to current directory.
#[test]
fn test() {
    assert_eq!(test_main().unwrap(), ());
}

fn test_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let factory = Factory::with_library_directory(
        std::env::current_dir()?
            .to_str()
            .ok_or(error::Error::Failed)?,
    )?
    .set_debug_message_handler(
        Box::new(|level, file, line, message| {
            println!("{:?}, {}, {}, {}", level, file, line, message);
        }),
        LogLevel::Error,
    );

    let c = factory.device_get_installed_count();
    println!("device count = {}", c);
    let device = factory.device_open(0)?;
    let serial = device.get_serialnum()?;
    let version = device.get_version()?;
    println!("serial = {} / hw ver = {:?}", serial, version);

    let color_control =
        device.get_color_control(ColorControlCommand::Brightness)?;
    println!("color control(brightness) = {:?}", color_control);

    let camera_config = DeviceConfiguration::builder().build();
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
