#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod calibration;
pub mod camera;
pub mod capture;
pub mod default;
pub mod device;
pub mod display;
pub mod error;
pub mod factory;
pub mod image;
pub mod imu;
pub mod k4a_functions;
pub mod transformation;
pub mod utility;

pub mod bindings {
    include!("bindings.rs");
}
