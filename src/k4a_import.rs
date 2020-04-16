use super::k4atypes_import::*;

pub(crate) type k4a_device_get_installed_count = fn() -> u32;
pub(crate) type k4a_set_debug_message_handler = fn(message_cb: k4a_logging_message_cb_t, message_cb_context: *mut (), min_level: k4a_log_level_t);
pub(crate) type k4a_set_allocator = fn(allocate: k4a_memory_allocate_cb_t, free: k4a_memory_destroy_cb_t) -> k4a_result_t;
pub(crate) type k4a_device_open = fn(index: u32, device_handle: *mut k4a_device_t) -> k4a_result_t;
pub(crate) type k4a_device_close = fn(device_handle: k4a_device_t);
pub(crate) type k4a_device_get_capture = fn(device_handle: k4a_device_t, capture_handle: k4a_capture_t, timeout_in_ms: i32) -> k4a_wait_result_t;
pub(crate) type k4a_device_get_imu_sample = fn(device_handle: k4a_device_t, imu_sample: *mut k4a_imu_sample_t, timeout_in_ms: i32) -> k4a_wait_result_t;
pub(crate) type k4a_capture_create = fn(capture_handle: *mut k4a_capture_t) -> k4a_result_t;
pub(crate) type k4a_capture_release = fn(capture_handle: k4a_capture_t);
pub(crate) type k4a_capture_reference = fn(capture_handle: k4a_capture_t);
pub(crate) type k4a_capture_get_color_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;
pub(crate) type k4a_capture_get_depth_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;
pub(crate) type k4a_capture_get_ir_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;
pub(crate) type k4a_capture_set_color_image = fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);
pub(crate) type k4a_capture_set_depth_image = fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);
pub(crate) type k4a_capture_set_ir_image = fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);
pub(crate) type k4a_capture_set_temperature_c = fn(capture_handle: k4a_capture_t, temperature_c: f32);
pub(crate) type k4a_capture_get_temperature_c = fn(capture_handle: k4a_capture_t) -> f32;
pub(crate) type k4a_image_create = fn(format: k4a_image_format_t, width_pixels: i32, height_pixels: i32, stride_bytes: i32, image_handle: *mut k4a_image_t) -> k4a_result_t;
pub(crate) type k4a_image_create_from_buffer = fn(format: k4a_image_format_t, width_pixels: i32, height_pixels: i32, stride_bytes: i32, buffer: *mut u8, buffer_size: usize, buffer_release_cb: *mut k4a_memory_destroy_cb_t, buffer_release_cb_context: *mut (), image_handle: *mut k4a_image_t) -> k4a_result_t;
pub(crate) type k4a_image_get_buffer = fn(image_handle: k4a_image_t) -> *mut u8;
pub(crate) type k4a_image_get_size = fn(image_handle: k4a_image_t) -> usize;
pub(crate) type k4a_image_get_format = fn(image_handle: k4a_image_t) -> k4a_image_format_t;
pub(crate) type k4a_image_get_width_pixels = fn(image_handle: k4a_image_t) -> i32;
pub(crate) type k4a_image_get_height_pixels = fn(image_handle: k4a_image_t) -> i32;
pub(crate) type k4a_image_get_stride_bytes = fn(image_handle: k4a_image_t) -> i32;


#[test]
pub fn test() {
    let n:k4a_device_t = std::ptr::null_mut();
    let x = k4a_result_t::K4A_RESULT_SUCCEEDED;
}
