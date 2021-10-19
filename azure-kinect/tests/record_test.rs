use azure_kinect::*;

//  Copy depthengine_2_0.dll and k4a.dll, k4arecord.dll to current directory.
#[test]
fn record_test() {
    assert_eq!(record_test_main().unwrap(), ());
}

fn record_test_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let factory = FactoryRecord::with_library_directory(
        std::env::current_dir()?
            .to_str()
            .ok_or(error::Error::Failed)?,
    )?
    .set_debug_message_handler(
        &|level, file, line, message| {
            println!("{:?}, {}, {}, {}", level, file, line, message);
        },
        LogLevel::Error,
    );

    let c = factory.core.device_get_installed_count();
    println!("device count = {}", c);
    let device = factory.core.device_open(0)?;
    let camera_config = DeviceConfiguration::builder()
        .depth_mode(DepthMode::WFov2x2Binned)
        .build();

    let record = factory.record_create("test.mkv", &device, &camera_config)?;
    record.add_imu_track()?;

    Ok(())
}
