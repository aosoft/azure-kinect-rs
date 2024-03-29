pub use calibration::Calibration;
pub use camera::Camera;
pub use capture::Capture;
pub use device::{Device, DeviceConfiguration, DeviceConfigurationBuilder};
pub use enums::*;
pub use error::Error;
pub use factory::{DebugMessageHandler, Factory, FactoryRecord, MemoryDestroyCallback};
pub use image::Image;
pub use imu::{Imu, ImuSample};
pub use structs::*;
pub use transformation::Transformation;
pub use vectors::*;

pub mod calibration;
pub mod camera;
pub mod capture;
pub mod device;
pub mod enums;
pub mod error;
pub mod factory;
pub mod image;
pub mod imu;
pub mod playback;
pub mod playback_data_block;
pub mod playback_track;
pub mod record;
pub mod structs;
pub mod transformation;
pub mod utility;
pub mod vectors;

pub trait NativeHandle {
    unsafe fn get_native_handle(&self) -> *mut ();
}