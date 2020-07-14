mod param;
mod recorder;
use crate::recorder::do_recording;
use azure_kinect::*;
use std::sync::atomic;

fn main() {
    std::process::exit(match main2() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    });
}

fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let param = param::Parameter::new()?;

    let factory = FactoryRecord::new()?;
    if param.list_device {
        list_devices(&factory);
    } else {
        let request_abort1 = std::sync::Arc::new(atomic::AtomicBool::new(false));
        let request_abort2 = request_abort1.clone();
        ctrlc::set_handler(move || {
            request_abort2.store(true, atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        do_recording(&factory, &param, || -> bool {
            request_abort1.load(atomic::Ordering::SeqCst)
        })?;
    }

    Ok(())
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
