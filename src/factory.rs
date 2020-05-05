use super::bindings::*;
use super::device::Device;
use super::error::{Error, ToResult};
use super::k4a_functions::*;
use std::ffi::c_void;
use std::ptr;

#[link(name = "kernel32")]
#[no_mangle]
extern "stdcall" {
    fn GetLastError() -> u32;
    fn LoadLibraryExW(
        lpLibFileName: *const u16,
        hFile: *const c_void,
        dwFlags: u32,
    ) -> *const c_void;
    fn FreeLibrary(hLibModule: *const c_void) -> i32;
    fn GetProcAddress(hModule: *const c_void, lpProcName: *const u8) -> *const c_void;
}

pub struct Factory {
    handle: *const c_void,

    pub(crate) k4a_device_get_installed_count: k4a_device_get_installed_count,
    pub(crate) k4a_set_debug_message_handler: k4a_set_debug_message_handler,
    pub(crate) k4a_set_allocator: k4a_set_allocator,
    pub(crate) k4a_device_open: k4a_device_open,
    pub(crate) k4a_device_close: k4a_device_close,
    pub(crate) k4a_device_get_capture: k4a_device_get_capture,
    pub(crate) k4a_device_get_imu_sample: k4a_device_get_imu_sample,
    pub(crate) k4a_capture_create: k4a_capture_create,
    pub(crate) k4a_capture_release: k4a_capture_release,
    pub(crate) k4a_capture_reference: k4a_capture_reference,
    pub(crate) k4a_capture_get_color_image: k4a_capture_get_color_image,
    pub(crate) k4a_capture_get_depth_image: k4a_capture_get_depth_image,
    pub(crate) k4a_capture_get_ir_image: k4a_capture_get_ir_image,
    pub(crate) k4a_capture_set_color_image: k4a_capture_set_color_image,
    pub(crate) k4a_capture_set_depth_image: k4a_capture_set_depth_image,
    pub(crate) k4a_capture_set_ir_image: k4a_capture_set_ir_image,
    pub(crate) k4a_capture_set_temperature_c: k4a_capture_set_temperature_c,
    pub(crate) k4a_capture_get_temperature_c: k4a_capture_get_temperature_c,
    pub(crate) k4a_image_create: k4a_image_create,
    pub(crate) k4a_image_create_from_buffer: k4a_image_create_from_buffer,
    pub(crate) k4a_image_get_buffer: k4a_image_get_buffer,
    pub(crate) k4a_image_get_size: k4a_image_get_size,
    pub(crate) k4a_image_get_format: k4a_image_get_format,
    pub(crate) k4a_image_get_width_pixels: k4a_image_get_width_pixels,
    pub(crate) k4a_image_get_height_pixels: k4a_image_get_height_pixels,
    pub(crate) k4a_image_get_stride_bytes: k4a_image_get_stride_bytes,
    pub(crate) k4a_image_get_timestamp_usec: k4a_image_get_timestamp_usec,
    pub(crate) k4a_image_get_device_timestamp_usec: k4a_image_get_device_timestamp_usec,
    pub(crate) k4a_image_get_system_timestamp_nsec: k4a_image_get_system_timestamp_nsec,
    pub(crate) k4a_image_get_exposure_usec: k4a_image_get_exposure_usec,
    pub(crate) k4a_image_get_white_balance: k4a_image_get_white_balance,
    pub(crate) k4a_image_get_iso_speed: k4a_image_get_iso_speed,
    pub(crate) k4a_image_set_device_timestamp_usec: k4a_image_set_device_timestamp_usec,
    pub(crate) k4a_image_set_timestamp_usec: k4a_image_set_timestamp_usec,
    pub(crate) k4a_image_set_system_timestamp_nsec: k4a_image_set_system_timestamp_nsec,
    pub(crate) k4a_image_set_exposure_usec: k4a_image_set_exposure_usec,
    pub(crate) k4a_image_set_exposure_time_usec: k4a_image_set_exposure_time_usec,
    pub(crate) k4a_image_set_white_balance: k4a_image_set_white_balance,
    pub(crate) k4a_image_set_iso_speed: k4a_image_set_iso_speed,
    pub(crate) k4a_image_reference: k4a_image_reference,
    pub(crate) k4a_image_release: k4a_image_release,
    pub(crate) k4a_device_start_cameras: k4a_device_start_cameras,
    pub(crate) k4a_device_stop_cameras: k4a_device_stop_cameras,
    pub(crate) k4a_device_start_imu: k4a_device_start_imu,
    pub(crate) k4a_device_stop_imu: k4a_device_stop_imu,
    pub(crate) k4a_device_get_serialnum: k4a_device_get_serialnum,
    pub(crate) k4a_device_get_version: k4a_device_get_version,
    pub(crate) k4a_device_get_color_control_capabilities: k4a_device_get_color_control_capabilities,
    pub(crate) k4a_device_get_color_control: k4a_device_get_color_control,
    pub(crate) k4a_device_set_color_control: k4a_device_set_color_control,
    pub(crate) k4a_device_get_raw_calibration: k4a_device_get_raw_calibration,
    pub(crate) k4a_device_get_calibration: k4a_device_get_calibration,
    pub(crate) k4a_device_get_sync_jack: k4a_device_get_sync_jack,
    pub(crate) k4a_calibration_get_from_raw: k4a_calibration_get_from_raw,
    pub(crate) k4a_calibration_3d_to_3d: k4a_calibration_3d_to_3d,
    pub(crate) k4a_calibration_2d_to_3d: k4a_calibration_2d_to_3d,
    pub(crate) k4a_calibration_3d_to_2d: k4a_calibration_3d_to_2d,
    pub(crate) k4a_calibration_2d_to_2d: k4a_calibration_2d_to_2d,
    pub(crate) k4a_calibration_color_2d_to_depth_2d: k4a_calibration_color_2d_to_depth_2d,
    pub(crate) k4a_transformation_create: k4a_transformation_create,
    pub(crate) k4a_transformation_destroy: k4a_transformation_destroy,
    pub(crate) k4a_transformation_depth_image_to_color_camera:
        k4a_transformation_depth_image_to_color_camera,
    pub(crate) k4a_transformation_depth_image_to_color_camera_custom:
        k4a_transformation_depth_image_to_color_camera_custom,
    pub(crate) k4a_transformation_color_image_to_depth_camera:
        k4a_transformation_color_image_to_depth_camera,
    pub(crate) k4a_transformation_depth_image_to_point_cloud:
        k4a_transformation_depth_image_to_point_cloud,
}

const k4a_libname: &'static str = "k4a.dll";

fn load_library(lib_dir: &str, dll_file_name: &str) -> Result<*const c_void, Error> {
    let full_path =
        std::path::Path::new(if lib_dir.len() > 0 { lib_dir } else { "." }).join(dll_file_name);

    unsafe {
        LoadLibraryExW(
            full_path
                .to_str()
                .ok_or(Error::Failed)?
                .encode_utf16()
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr(),
            ptr::null(),
            0x000,
        )
        .to_result()
    }
}

macro_rules! proc_address {
    ($h:ident, $proc_name:ident) => {
        std::mem::transmute::<_, $proc_name>(
            GetProcAddress($h, concat!(stringify!($proc_name), "\0").as_ptr()).to_result()?,
        )
    };
}

impl Factory {
    fn new(handle: *const c_void) -> Result<Factory, Error> {
        unsafe {
            Ok(Factory {
                handle: handle,
                k4a_device_get_installed_count: proc_address!(
                    handle,
                    k4a_device_get_installed_count
                ),
                k4a_set_debug_message_handler: proc_address!(handle, k4a_set_debug_message_handler),
                k4a_set_allocator: proc_address!(handle, k4a_set_allocator),
                k4a_device_open: proc_address!(handle, k4a_device_open),
                k4a_device_close: proc_address!(handle, k4a_device_close),
                k4a_device_get_capture: proc_address!(handle, k4a_device_get_capture),
                k4a_device_get_imu_sample: proc_address!(handle, k4a_device_get_imu_sample),
                k4a_capture_create: proc_address!(handle, k4a_capture_create),
                k4a_capture_release: proc_address!(handle, k4a_capture_release),
                k4a_capture_reference: proc_address!(handle, k4a_capture_reference),
                k4a_capture_get_color_image: proc_address!(handle, k4a_capture_get_color_image),
                k4a_capture_get_depth_image: proc_address!(handle, k4a_capture_get_depth_image),
                k4a_capture_get_ir_image: proc_address!(handle, k4a_capture_get_ir_image),
                k4a_capture_set_color_image: proc_address!(handle, k4a_capture_set_color_image),
                k4a_capture_set_depth_image: proc_address!(handle, k4a_capture_set_depth_image),
                k4a_capture_set_ir_image: proc_address!(handle, k4a_capture_set_ir_image),
                k4a_capture_set_temperature_c: proc_address!(handle, k4a_capture_set_temperature_c),
                k4a_capture_get_temperature_c: proc_address!(handle, k4a_capture_get_temperature_c),
                k4a_image_create: proc_address!(handle, k4a_image_create),
                k4a_image_create_from_buffer: proc_address!(handle, k4a_image_create_from_buffer),
                k4a_image_get_buffer: proc_address!(handle, k4a_image_get_buffer),
                k4a_image_get_size: proc_address!(handle, k4a_image_get_size),
                k4a_image_get_format: proc_address!(handle, k4a_image_get_format),
                k4a_image_get_width_pixels: proc_address!(handle, k4a_image_get_width_pixels),
                k4a_image_get_height_pixels: proc_address!(handle, k4a_image_get_height_pixels),
                k4a_image_get_stride_bytes: proc_address!(handle, k4a_image_get_stride_bytes),
                k4a_image_get_timestamp_usec: proc_address!(handle, k4a_image_get_timestamp_usec),
                k4a_image_get_device_timestamp_usec: proc_address!(
                    handle,
                    k4a_image_get_device_timestamp_usec
                ),
                k4a_image_get_system_timestamp_nsec: proc_address!(
                    handle,
                    k4a_image_get_system_timestamp_nsec
                ),
                k4a_image_get_exposure_usec: proc_address!(handle, k4a_image_get_exposure_usec),
                k4a_image_get_white_balance: proc_address!(handle, k4a_image_get_white_balance),
                k4a_image_get_iso_speed: proc_address!(handle, k4a_image_get_iso_speed),
                k4a_image_set_device_timestamp_usec: proc_address!(
                    handle,
                    k4a_image_set_device_timestamp_usec
                ),
                k4a_image_set_timestamp_usec: proc_address!(handle, k4a_image_set_timestamp_usec),
                k4a_image_set_system_timestamp_nsec: proc_address!(
                    handle,
                    k4a_image_set_system_timestamp_nsec
                ),
                k4a_image_set_exposure_usec: proc_address!(handle, k4a_image_set_exposure_usec),
                k4a_image_set_exposure_time_usec: proc_address!(
                    handle,
                    k4a_image_set_exposure_time_usec
                ),
                k4a_image_set_white_balance: proc_address!(handle, k4a_image_set_white_balance),
                k4a_image_set_iso_speed: proc_address!(handle, k4a_image_set_iso_speed),
                k4a_image_reference: proc_address!(handle, k4a_image_reference),
                k4a_image_release: proc_address!(handle, k4a_image_release),
                k4a_device_start_cameras: proc_address!(handle, k4a_device_start_cameras),
                k4a_device_stop_cameras: proc_address!(handle, k4a_device_stop_cameras),
                k4a_device_start_imu: proc_address!(handle, k4a_device_start_imu),
                k4a_device_stop_imu: proc_address!(handle, k4a_device_stop_imu),
                k4a_device_get_serialnum: proc_address!(handle, k4a_device_get_serialnum),
                k4a_device_get_version: proc_address!(handle, k4a_device_get_version),
                k4a_device_get_color_control_capabilities: proc_address!(
                    handle,
                    k4a_device_get_color_control_capabilities
                ),
                k4a_device_get_color_control: proc_address!(handle, k4a_device_get_color_control),
                k4a_device_set_color_control: proc_address!(handle, k4a_device_set_color_control),
                k4a_device_get_raw_calibration: proc_address!(
                    handle,
                    k4a_device_get_raw_calibration
                ),
                k4a_device_get_calibration: proc_address!(handle, k4a_device_get_calibration),
                k4a_device_get_sync_jack: proc_address!(handle, k4a_device_get_sync_jack),
                k4a_calibration_get_from_raw: proc_address!(handle, k4a_calibration_get_from_raw),
                k4a_calibration_3d_to_3d: proc_address!(handle, k4a_calibration_3d_to_3d),
                k4a_calibration_2d_to_3d: proc_address!(handle, k4a_calibration_2d_to_3d),
                k4a_calibration_3d_to_2d: proc_address!(handle, k4a_calibration_3d_to_2d),
                k4a_calibration_2d_to_2d: proc_address!(handle, k4a_calibration_2d_to_2d),
                k4a_calibration_color_2d_to_depth_2d: proc_address!(
                    handle,
                    k4a_calibration_color_2d_to_depth_2d
                ),
                k4a_transformation_create: proc_address!(handle, k4a_transformation_create),
                k4a_transformation_destroy: proc_address!(handle, k4a_transformation_destroy),
                k4a_transformation_depth_image_to_color_camera: proc_address!(
                    handle,
                    k4a_transformation_depth_image_to_color_camera
                ),
                k4a_transformation_depth_image_to_color_camera_custom: proc_address!(
                    handle,
                    k4a_transformation_depth_image_to_color_camera_custom
                ),
                k4a_transformation_color_image_to_depth_camera: proc_address!(
                    handle,
                    k4a_transformation_color_image_to_depth_camera
                ),
                k4a_transformation_depth_image_to_point_cloud: proc_address!(
                    handle,
                    k4a_transformation_depth_image_to_point_cloud
                ),
            })
        }
    }

    pub fn load(lib_dir: &str) -> Result<Factory, Error> {
        let h = load_library(lib_dir, k4a_libname)?;
        let r = Factory::new(h);
        if let Err(e) = r {
            unsafe {
                FreeLibrary(h);
            }
        }
        r
    }

    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        unsafe { (self.k4a_device_get_installed_count)() }
    }

    /// Open a k4a device.
    pub unsafe fn device_open(&self, index: u32) -> Result<Device, Error> {
        unsafe {
            let mut handle: k4a_device_t = ptr::null_mut();
            Error::from((self.k4a_device_open)(index, &mut handle))
                .to_result_fn(&|| Device::new(self, handle))
        }
    }
}

impl Drop for Factory {
    fn drop(&mut self) {
        if self.handle != ptr::null() {
            unsafe {
                FreeLibrary(self.handle);
                self.handle = ptr::null();
            }
        }
    }
}

impl ToResult for *const c_void {
    fn to_result(&self) -> Result<*const c_void, Error> {
        if *self == ptr::null() {
            unsafe { Err(Error::Win32Error(GetLastError())) }
        } else {
            Ok(*self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> std::result::Result<(), Box<std::error::Error>> {
        let manager = Factory::load(std::env::current_dir()?.to_str().ok_or(Error::Failed)?);
        assert!(manager.is_ok());
        let manager2 = manager.unwrap();
        let c = (manager2.k4a_device_get_installed_count)();
        println!("device count = {}", c);
        Ok(())
    }
}
