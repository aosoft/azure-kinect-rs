use super::bindings::*;
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

pub(crate) struct DllManager {
    handle: *const c_void,

    pub k4a_device_get_installed_count: k4a_device_get_installed_count,
    pub k4a_set_debug_message_handler: k4a_set_debug_message_handler,
    pub k4a_set_allocator: k4a_set_allocator,
    pub k4a_device_open: k4a_device_open,
    pub k4a_device_close: k4a_device_close,
    pub k4a_device_get_capture: k4a_device_get_capture,
    pub k4a_device_get_imu_sample: k4a_device_get_imu_sample,
    pub k4a_capture_create: k4a_capture_create,
    pub k4a_capture_release: k4a_capture_release,
    pub k4a_capture_reference: k4a_capture_reference,
    pub k4a_capture_get_color_image: k4a_capture_get_color_image,
    pub k4a_capture_get_depth_image: k4a_capture_get_depth_image,
    pub k4a_capture_get_ir_image: k4a_capture_get_ir_image,
    pub k4a_capture_set_color_image: k4a_capture_set_color_image,
    pub k4a_capture_set_depth_image: k4a_capture_set_depth_image,
    pub k4a_capture_set_ir_image: k4a_capture_set_ir_image,
    pub k4a_capture_set_temperature_c: k4a_capture_set_temperature_c,
    pub k4a_capture_get_temperature_c: k4a_capture_get_temperature_c,
    pub k4a_image_create: k4a_image_create,
    pub k4a_image_create_from_buffer: k4a_image_create_from_buffer,
    pub k4a_image_get_buffer: k4a_image_get_buffer,
    pub k4a_image_get_size: k4a_image_get_size,
    pub k4a_image_get_format: k4a_image_get_format,
    pub k4a_image_get_width_pixels: k4a_image_get_width_pixels,
    pub k4a_image_get_height_pixels: k4a_image_get_height_pixels,
    pub k4a_image_get_stride_bytes: k4a_image_get_stride_bytes,
    pub k4a_image_get_timestamp_usec: k4a_image_get_timestamp_usec,
    pub k4a_image_get_device_timestamp_usec: k4a_image_get_device_timestamp_usec,
    pub k4a_image_get_system_timestamp_nsec: k4a_image_get_system_timestamp_nsec,
    pub k4a_image_get_exposure_usec: k4a_image_get_exposure_usec,
    pub k4a_image_get_white_balance: k4a_image_get_white_balance,
    pub k4a_image_get_iso_speed: k4a_image_get_iso_speed,
    pub k4a_image_set_device_timestamp_usec: k4a_image_set_device_timestamp_usec,
    pub k4a_image_set_timestamp_usec: k4a_image_set_timestamp_usec,
    pub k4a_image_set_system_timestamp_nsec: k4a_image_set_system_timestamp_nsec,
    pub k4a_image_set_exposure_usec: k4a_image_set_exposure_usec,
    pub k4a_image_set_exposure_time_usec: k4a_image_set_exposure_time_usec,
    pub k4a_image_set_white_balance: k4a_image_set_white_balance,
    pub k4a_image_set_iso_speed: k4a_image_set_iso_speed,
    pub k4a_image_reference: k4a_image_reference,
    pub k4a_image_release: k4a_image_release,
    pub k4a_device_start_cameras: k4a_device_start_cameras,
    pub k4a_device_stop_cameras: k4a_device_stop_cameras,
    pub k4a_device_start_imu: k4a_device_start_imu,
    pub k4a_device_stop_imu: k4a_device_stop_imu,
    pub k4a_device_get_serialnum: k4a_device_get_serialnum,
    pub k4a_device_get_version: k4a_device_get_version,
    pub k4a_device_get_color_control_capabilities: k4a_device_get_color_control_capabilities,
    pub k4a_device_get_color_control: k4a_device_get_color_control,
    pub k4a_device_set_color_control: k4a_device_set_color_control,
    pub k4a_device_get_raw_calibration: k4a_device_get_raw_calibration,
    pub k4a_device_get_calibration: k4a_device_get_calibration,
    pub k4a_device_get_sync_jack: k4a_device_get_sync_jack,
    pub k4a_calibration_get_from_raw: k4a_calibration_get_from_raw,
    pub k4a_calibration_3d_to_3d: k4a_calibration_3d_to_3d,
    pub k4a_calibration_2d_to_3d: k4a_calibration_2d_to_3d,
    pub k4a_calibration_3d_to_2d: k4a_calibration_3d_to_2d,
    pub k4a_calibration_2d_to_2d: k4a_calibration_2d_to_2d,
    pub k4a_calibration_color_2d_to_depth_2d: k4a_calibration_color_2d_to_depth_2d,
    pub k4a_transformation_create: k4a_transformation_create,
    pub k4a_transformation_destroy: k4a_transformation_destroy,
    pub k4a_transformation_depth_image_to_color_camera:
        k4a_transformation_depth_image_to_color_camera,
    pub k4a_transformation_depth_image_to_color_camera_custom:
        k4a_transformation_depth_image_to_color_camera_custom,
    pub k4a_transformation_color_image_to_depth_camera:
        k4a_transformation_color_image_to_depth_camera,
    pub k4a_transformation_depth_image_to_point_cloud:
        k4a_transformation_depth_image_to_point_cloud,
}

const k4a_libname: &'static str = "k4a.dll";

fn load_library(path: &str, dll_file_name: &str) -> Result<*const c_void, Error> {
    let full_path =
        std::path::Path::new(if path.len() > 0 { path } else { "." }).join(dll_file_name);

    unsafe {
        LoadLibraryExW(
            full_path
                .to_str()
                .ok_or(Error(0))?
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
        std::mem::transmute::<_, $proc_name>(GetProcAddress(
            $h,
            concat!(stringify!($proc_name), "\0").as_ptr()
        ).to_result()?)
    };
}

impl DllManager {
    fn new(handle: *const c_void) -> Result<DllManager, Error> {
        unsafe {
            Ok(DllManager {
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

    fn load(path: &str) -> Result<DllManager, Error> {
        let h = load_library(path, k4a_libname)?;
        let r = DllManager::new(h);
        if let Err(e) = r {
            unsafe {
                FreeLibrary(h);
            }
        }
        r
    }
}

impl Drop for DllManager {
    fn drop(&mut self) {
        if self.handle != ptr::null() {
            unsafe {
                FreeLibrary(self.handle);
                self.handle = ptr::null();
            }
        }
    }
}

static mut dllmanager: Option<DllManager> = Option::<DllManager>::None;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Error(u32);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

trait ToResult: Sized {
    fn to_result(&self) -> Result<Self, Error>;
}

impl ToResult for *const c_void {
    fn to_result(&self) -> Result<*const c_void, Error> {
        if *self == ptr::null() {
            unsafe { Err(Error(GetLastError())) }
        } else {
            Ok(*self)
        }
    }
}
