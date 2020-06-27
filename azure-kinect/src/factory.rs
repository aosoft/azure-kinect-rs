use super::error::ToResult;
use super::k4a_functions::*;
use super::*;
use crate::playback::Playback;
use crate::record::Record;
use std::ffi::{c_void, CString};
use std::os::raw;
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
    fn GetModuleHandleW(lpModuleName: *const u16) -> *const c_void;
}

pub type DebugMessageHandler = Box<dyn Fn(k4a_log_level_t, &str, raw::c_int, &str)>;

pub struct Factory {
    handle: *const c_void,
    require_free_library: bool,
    debug_message_handler: Option<DebugMessageHandler>,

    pub(crate) k4a_device_get_installed_count: k4a_device_get_installed_count,
    pub(crate) k4a_set_debug_message_handler: k4a_set_debug_message_handler,
    //pub(crate) k4a_set_allocator: k4a_set_allocator,
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
    //pub(crate) k4a_image_get_timestamp_usec: k4a_image_get_timestamp_usec,
    pub(crate) k4a_image_get_device_timestamp_usec: k4a_image_get_device_timestamp_usec,
    pub(crate) k4a_image_get_system_timestamp_nsec: k4a_image_get_system_timestamp_nsec,
    pub(crate) k4a_image_get_exposure_usec: k4a_image_get_exposure_usec,
    pub(crate) k4a_image_get_white_balance: k4a_image_get_white_balance,
    pub(crate) k4a_image_get_iso_speed: k4a_image_get_iso_speed,
    pub(crate) k4a_image_set_device_timestamp_usec: k4a_image_set_device_timestamp_usec,
    //pub(crate) k4a_image_set_timestamp_usec: k4a_image_set_timestamp_usec,
    pub(crate) k4a_image_set_system_timestamp_nsec: k4a_image_set_system_timestamp_nsec,
    pub(crate) k4a_image_set_exposure_usec: k4a_image_set_exposure_usec,
    //pub(crate) k4a_image_set_exposure_time_usec: k4a_image_set_exposure_time_usec,
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

const K4A_LIBNAME: &'static str = "k4a.dll";
const K4ARECORD_LIBNAME: &'static str = "k4arecord.dll";

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
    fn with_handle(handle: *const c_void, require_free_library: bool) -> Result<Factory, Error> {
        unsafe {
            Ok(Factory {
                handle: handle,
                require_free_library: require_free_library,
                debug_message_handler: None,
                k4a_device_get_installed_count: proc_address!(
                    handle,
                    k4a_device_get_installed_count
                ),
                k4a_set_debug_message_handler: proc_address!(handle, k4a_set_debug_message_handler),
                //k4a_set_allocator: proc_address!(handle, k4a_set_allocator),
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
                //k4a_image_get_timestamp_usec: proc_address!(handle, k4a_image_get_timestamp_usec),
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
                //k4a_image_set_timestamp_usec: proc_address!(handle, k4a_image_set_timestamp_usec),
                k4a_image_set_system_timestamp_nsec: proc_address!(
                    handle,
                    k4a_image_set_system_timestamp_nsec
                ),
                k4a_image_set_exposure_usec: proc_address!(handle, k4a_image_set_exposure_usec),
                //k4a_image_set_exposure_time_usec: proc_address!(
                //    handle,
                //    k4a_image_set_exposure_time_usec
                //),
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

    pub fn new() -> Result<Factory, Error> {
        Ok(Self::with_library_directory(
            std::env::current_exe()
                .map_err(|_| Error::Failed)?
                .parent()
                .ok_or(Error::Failed)?
                .to_str()
                .ok_or(Error::Failed)?,
        )?)
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<Factory, Error> {
        let h = load_library(lib_dir, K4A_LIBNAME)?;
        let r = Factory::with_handle(h, true);
        if let Err(_) = r {
            unsafe {
                FreeLibrary(h);
            }
        }
        r
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn set_debug_message_handler(
        mut self,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) -> Self {
        self.set_debug_message_handler_internal(debug_message_handler, min_level);
        self
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler(mut self) -> Self {
        self.reset_debug_message_handler_internal();
        self
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub(crate) fn set_debug_message_handler_internal(
        &mut self,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) {
        self.debug_message_handler = debug_message_handler.into();
        (self.k4a_set_debug_message_handler)(
            Some(debug_message_handler_func),
            &self.debug_message_handler as *const Option<DebugMessageHandler> as *mut (),
            min_level,
        );
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler_internal(&mut self) {
        self.debug_message_handler = None;
        (self.k4a_set_debug_message_handler)(
            None,
            ptr::null_mut(),
            k4a_log_level_t::K4A_LOG_LEVEL_OFF,
        );
    }

    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        (self.k4a_device_get_installed_count)()
    }

    /// Open a k4a device.
    pub fn device_open(&self, index: u32) -> Result<Device, Error> {
        let mut handle: k4a_device_t = ptr::null_mut();
        Error::from((self.k4a_device_open)(index, &mut handle))
            .to_result_fn(&|| Device::from_handle(self, handle))
    }
}

extern "C" fn debug_message_handler_func(
    context: *mut ::std::os::raw::c_void,
    level: k4a_log_level_t,
    file: *const ::std::os::raw::c_char,
    line: ::std::os::raw::c_int,
    message: *const ::std::os::raw::c_char,
) {
    unsafe {
        let h = context as *const Option<DebugMessageHandler>;
        if h != ptr::null() && (*h).is_some() {
            (*h).as_ref().unwrap()(
                level,
                std::ffi::CStr::from_ptr(file).to_str().unwrap_or_default(),
                line,
                std::ffi::CStr::from_ptr(message)
                    .to_str()
                    .unwrap_or_default(),
            );
        }
    }
}

impl Drop for Factory {
    fn drop(&mut self) {
        if self.handle != ptr::null() && self.require_free_library {
            unsafe {
                FreeLibrary(self.handle);
                self.handle = ptr::null();
            }
        }
    }
}

pub struct FactoryRecord {
    handle: *const c_void,
    pub(crate) k4a: Factory,
    pub(crate) k4a_playback_open: k4a_playback_open,
    pub(crate) k4a_playback_get_raw_calibration: k4a_playback_get_raw_calibration,
    pub(crate) k4a_playback_get_calibration: k4a_playback_get_calibration,
    pub(crate) k4a_playback_get_record_configuration: k4a_playback_get_record_configuration,
    pub(crate) k4a_playback_check_track_exists: k4a_playback_check_track_exists,
    pub(crate) k4a_playback_get_track_count: k4a_playback_get_track_count,
    pub(crate) k4a_playback_get_track_name: k4a_playback_get_track_name,
    pub(crate) k4a_playback_track_is_builtin: k4a_playback_track_is_builtin,
    pub(crate) k4a_playback_track_get_video_settings: k4a_playback_track_get_video_settings,
    pub(crate) k4a_playback_track_get_codec_id: k4a_playback_track_get_codec_id,
    pub(crate) k4a_playback_track_get_codec_context: k4a_playback_track_get_codec_context,
    pub(crate) k4a_playback_get_tag: k4a_playback_get_tag,
    pub(crate) k4a_playback_set_color_conversion: k4a_playback_set_color_conversion,
    pub(crate) k4a_playback_get_attachment: k4a_playback_get_attachment,
    pub(crate) k4a_playback_get_next_capture: k4a_playback_get_next_capture,
    pub(crate) k4a_playback_get_previous_capture: k4a_playback_get_previous_capture,
    pub(crate) k4a_playback_get_next_imu_sample: k4a_playback_get_next_imu_sample,
    pub(crate) k4a_playback_get_previous_imu_sample: k4a_playback_get_previous_imu_sample,
    pub(crate) k4a_playback_get_next_data_block: k4a_playback_get_next_data_block,
    pub(crate) k4a_playback_get_previous_data_block: k4a_playback_get_previous_data_block,
    pub(crate) k4a_playback_data_block_get_device_timestamp_usec:
        k4a_playback_data_block_get_device_timestamp_usec,
    pub(crate) k4a_playback_data_block_get_buffer_size: k4a_playback_data_block_get_buffer_size,
    pub(crate) k4a_playback_data_block_get_buffer: k4a_playback_data_block_get_buffer,
    pub(crate) k4a_playback_data_block_release: k4a_playback_data_block_release,
    pub(crate) k4a_playback_seek_timestamp: k4a_playback_seek_timestamp,
    pub(crate) k4a_playback_get_recording_length_usec: k4a_playback_get_recording_length_usec,
    //pub(crate) k4a_playback_get_last_timestamp_usec: k4a_playback_get_last_timestamp_usec,
    pub(crate) k4a_playback_close: k4a_playback_close,
    pub(crate) k4a_record_create: k4a_record_create,
    pub(crate) k4a_record_add_tag: k4a_record_add_tag,
    pub(crate) k4a_record_add_imu_track: k4a_record_add_imu_track,
    pub(crate) k4a_record_add_attachment: k4a_record_add_attachment,
    pub(crate) k4a_record_add_custom_video_track: k4a_record_add_custom_video_track,
    pub(crate) k4a_record_add_custom_subtitle_track: k4a_record_add_custom_subtitle_track,
    pub(crate) k4a_record_write_header: k4a_record_write_header,
    pub(crate) k4a_record_write_capture: k4a_record_write_capture,
    pub(crate) k4a_record_write_imu_sample: k4a_record_write_imu_sample,
    pub(crate) k4a_record_write_custom_track_data: k4a_record_write_custom_track_data,
    pub(crate) k4a_record_flush: k4a_record_flush,
    pub(crate) k4a_record_close: k4a_record_close,
}

impl FactoryRecord {
    fn with_handle(handle: *const c_void, k4a: Factory) -> Result<FactoryRecord, Error> {
        unsafe {
            Ok(FactoryRecord {
                handle: handle,
                k4a: k4a,
                k4a_playback_open: proc_address!(handle, k4a_playback_open),
                k4a_playback_get_raw_calibration: proc_address!(
                    handle,
                    k4a_playback_get_raw_calibration
                ),
                k4a_playback_get_calibration: proc_address!(handle, k4a_playback_get_calibration),
                k4a_playback_get_record_configuration: proc_address!(
                    handle,
                    k4a_playback_get_record_configuration
                ),
                k4a_playback_check_track_exists: proc_address!(
                    handle,
                    k4a_playback_check_track_exists
                ),
                k4a_playback_get_track_count: proc_address!(handle, k4a_playback_get_track_count),
                k4a_playback_get_track_name: proc_address!(handle, k4a_playback_get_track_name),
                k4a_playback_track_is_builtin: proc_address!(handle, k4a_playback_track_is_builtin),
                k4a_playback_track_get_video_settings: proc_address!(
                    handle,
                    k4a_playback_track_get_video_settings
                ),
                k4a_playback_track_get_codec_id: proc_address!(
                    handle,
                    k4a_playback_track_get_codec_id
                ),
                k4a_playback_track_get_codec_context: proc_address!(
                    handle,
                    k4a_playback_track_get_codec_context
                ),
                k4a_playback_get_tag: proc_address!(handle, k4a_playback_get_tag),
                k4a_playback_set_color_conversion: proc_address!(
                    handle,
                    k4a_playback_set_color_conversion
                ),
                k4a_playback_get_attachment: proc_address!(handle, k4a_playback_get_attachment),
                k4a_playback_get_next_capture: proc_address!(handle, k4a_playback_get_next_capture),
                k4a_playback_get_previous_capture: proc_address!(
                    handle,
                    k4a_playback_get_previous_capture
                ),
                k4a_playback_get_next_imu_sample: proc_address!(
                    handle,
                    k4a_playback_get_next_imu_sample
                ),
                k4a_playback_get_previous_imu_sample: proc_address!(
                    handle,
                    k4a_playback_get_previous_imu_sample
                ),
                k4a_playback_get_next_data_block: proc_address!(
                    handle,
                    k4a_playback_get_next_data_block
                ),
                k4a_playback_get_previous_data_block: proc_address!(
                    handle,
                    k4a_playback_get_previous_data_block
                ),
                k4a_playback_data_block_get_device_timestamp_usec: proc_address!(
                    handle,
                    k4a_playback_data_block_get_device_timestamp_usec
                ),
                k4a_playback_data_block_get_buffer_size: proc_address!(
                    handle,
                    k4a_playback_data_block_get_buffer_size
                ),
                k4a_playback_data_block_get_buffer: proc_address!(
                    handle,
                    k4a_playback_data_block_get_buffer
                ),
                k4a_playback_data_block_release: proc_address!(
                    handle,
                    k4a_playback_data_block_release
                ),
                k4a_playback_seek_timestamp: proc_address!(handle, k4a_playback_seek_timestamp),
                k4a_playback_get_recording_length_usec: proc_address!(
                    handle,
                    k4a_playback_get_recording_length_usec
                ),
                //k4a_playback_get_last_timestamp_usec : proc_address!(handle, k4a_playback_get_last_timestamp_usec),
                k4a_playback_close: proc_address!(handle, k4a_playback_close),
                k4a_record_create: proc_address!(handle, k4a_record_create),
                k4a_record_add_tag: proc_address!(handle, k4a_record_add_tag),
                k4a_record_add_imu_track: proc_address!(handle, k4a_record_add_imu_track),
                k4a_record_add_attachment: proc_address!(handle, k4a_record_add_attachment),
                k4a_record_add_custom_video_track: proc_address!(
                    handle,
                    k4a_record_add_custom_video_track
                ),
                k4a_record_add_custom_subtitle_track: proc_address!(
                    handle,
                    k4a_record_add_custom_subtitle_track
                ),
                k4a_record_write_header: proc_address!(handle, k4a_record_write_header),
                k4a_record_write_capture: proc_address!(handle, k4a_record_write_capture),
                k4a_record_write_imu_sample: proc_address!(handle, k4a_record_write_imu_sample),
                k4a_record_write_custom_track_data: proc_address!(
                    handle,
                    k4a_record_write_custom_track_data
                ),
                k4a_record_flush: proc_address!(handle, k4a_record_flush),
                k4a_record_close: proc_address!(handle, k4a_record_close),
            })
        }
    }

    pub fn new() -> Result<FactoryRecord, Error> {
        Ok(Self::with_library_directory(
            std::env::current_exe()
                .map_err(|_| Error::Failed)?
                .parent()
                .ok_or(Error::Failed)?
                .to_str()
                .ok_or(Error::Failed)?,
        )?)
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<FactoryRecord, Error> {
        let h = load_library(lib_dir, K4ARECORD_LIBNAME)?;
        let h2 = unsafe {
            GetModuleHandleW(
                K4A_LIBNAME
                    .encode_utf16()
                    .chain(Some(0))
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            )
        };
        let r = FactoryRecord::with_handle(h, Factory::with_handle(h2, false)?);
        if let Err(_) = r {
            unsafe {
                FreeLibrary(h);
            }
        }
        r
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn set_debug_message_handler(
        mut self,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) -> Self {
        self.k4a
            .set_debug_message_handler_internal(debug_message_handler, min_level);
        self
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler(mut self) -> Self {
        self.k4a.reset_debug_message_handler_internal();
        self
    }

    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        self.k4a.device_get_installed_count()
    }

    /// Open a k4a device.
    pub fn device_open(&self, index: u32) -> Result<Device, Error> {
        self.k4a.device_open(index)
    }

    /// Opens a K4A recording for playback.
    pub fn playback_open(&self, path: &str) -> Result<Playback, Error> {
        let mut handle: k4a_playback_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from((self.k4a_playback_open)(path.as_ptr(), &mut handle))
            .to_result_fn(&|| Playback::from_handle(self, handle))
    }

    /// Opens a new recording file for writing
    pub fn record_create(
        &self,
        path: &str,
        device: &Device,
        device_configuration: &k4a_device_configuration_t,
    ) -> Result<Record, Error> {
        let mut handle: k4a_record_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from((self.k4a_record_create)(
            path.as_ptr(),
            device.handle,
            *device_configuration,
            &mut handle,
        ))
        .to_result_fn(&|| Record::from_handle(self, handle))
    }
}

impl Drop for FactoryRecord {
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
    fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = Factory::with_library_directory(
            std::env::current_dir()?.to_str().ok_or(Error::Failed)?,
        );
        assert!(manager.is_ok());
        let manager2 = manager.unwrap();
        let c = (manager2.k4a_device_get_installed_count)();
        println!("device count = {}", c);
        Ok(())
    }
}
