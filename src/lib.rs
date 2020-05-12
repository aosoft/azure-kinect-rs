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
pub mod camera;
pub mod imu;
pub mod image;
pub mod transformation;

pub mod bindings {
    include!("bindings.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::bindings::*;

    extern "C" fn debug_message_handler(
        _: *mut ::std::os::raw::c_void,
        level: k4a_log_level_t,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    ){
        unsafe {
            let file2 = std::ffi::CStr::from_ptr(file).to_str().unwrap_or("");
            let message2 = std::ffi::CStr::from_ptr(message).to_str().unwrap_or("");

            println!("{:?}, {}, {}, {}", level, file2, line, message2);
        }
    }

    #[test]
    fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let factory = factory::Factory::load(
            std::env::current_dir()?
                .to_str()
                .ok_or(error::Error::Failed)?,
        )?;

        factory.set_debug_message_handler(
            Some(debug_message_handler),
            std::ptr::null_mut(),
            k4a_log_level_t::K4A_LOG_LEVEL_ERROR)?;

        let c = factory.device_get_installed_count();
        println!("device count = {}", c);
        let device = factory.device_open(0)?;
        let serial = device.get_serialnum()?;
        let version = device.get_version();
        println!("serial = {} / hw ver = {:?}", serial, version);

        let color_control = device.get_color_control(
            bindings::k4a_color_control_command_t::K4A_COLOR_CONTROL_BRIGHTNESS,
        )?;
        println!("color control(brightness) = {:?}", color_control);

        let camera_config = k4a_device_configuration_t::default();
        let camera = device.start_cameras(&camera_config)?;
        let imu = camera.start_imu()?;
        let imu_sample = imu.get_imu_sample_wait_infinite()?;
        println!("imu = {}", imu_sample);

        Ok(())
    }
}
