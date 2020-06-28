use azure_kinect::*;

//  Copy depthengine_2_0.dll and k4a.dll, k4arecord.dll to current directory.
#[test]
fn record_test() {
    let r = record_test_main();
}

fn record_test_main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let factory = FactoryRecord::with_library_directory(
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
    let camera_config = k4a_device_configuration_t {
        depth_mode: k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_2X2BINNED,
        ..k4a_device_configuration_t::default()
    };
    let record = factory.record_create("test.mkv", &device,&camera_config)?;
    record.add_imu_track();

    Ok(())
}
