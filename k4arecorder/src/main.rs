mod param;
mod recorder;
use azure_kinect::*;

fn main() {
    let factory = FactoryRecord::new().unwrap();
    list_devices(&factory);
}

fn list_devices(factory: &FactoryRecord) {
    let device_count = factory.device_get_installed_count();
    if device_count > 0 {
        for i in 0..device_count {
            println!("Index:{}", i);
            if let Ok(device) = factory.device_open(i) {
                match device.get_serialnum() {
                    Ok(s) => println!("\tSerial:{}", s),
                    Err(_) => println!("\tSerial:ERROR"),
                };
                if let Ok(version) = device.get_version() {
                    println!("\tColor:{}", version.rgb);
                    println!("\tDepth:{}", version.depth);
                }
            } else {
                println!("{}\tDevice Open Failed", i);
            }
        }
    } else {
        println!("No devices connected.");
    }
}
