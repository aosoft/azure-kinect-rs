#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod error;
pub mod default;
pub mod display;
pub mod k4a_functions;
pub mod utility;
pub mod factory;
pub mod calibration;
pub mod capture;
pub mod device;
pub mod image;
pub mod transformation;

pub mod bindings {
    include!("bindings.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let factory = factory::Factory::load(
            std::env::current_dir()?
                .to_str()
                .ok_or(error::Error::Failed)?,
        )?;
        let c = factory.device_get_installed_count();
        println!("device count = {}", c);
        let device = factory.device_open(0)?;
        let serial = device.get_serialnum()?;
        let version = device.get_version();
        println!("serial = {} / hw ver = {:?}", serial, version);

        let imu = device.get_imu_sample_wait_infinite()?;
        println!("imu = {}", imu);
        let color_control = device.get_color_control(
            bindings::k4a_color_control_command_t::K4A_COLOR_CONTROL_BRIGHTNESS,
        )?;
        println!("color control(brightness) = {:?}", color_control);

        Ok(())
    }
}
