use crate::bindings::*;

pub(crate) type k4a_device_get_installed_count = fn() -> u32;

pub(crate) type k4a_set_debug_message_handler = fn(
    message_cb: k4a_logging_message_cb_t,
    message_cb_context: *mut (),
    min_level: k4a_log_level_t,
) -> k4a_result_t;

//pub(crate) type k4a_set_allocator =
//    fn(allocate: k4a_memory_allocate_cb_t, free: k4a_memory_destroy_cb_t) -> k4a_result_t;

pub(crate) type k4a_device_open = fn(index: u32, device_handle: *mut k4a_device_t) -> k4a_result_t;

pub(crate) type k4a_device_close = fn(device_handle: k4a_device_t);

pub(crate) type k4a_device_get_capture = fn(
    device_handle: k4a_device_t,
    capture_handle: *mut k4a_capture_t,
    timeout_in_ms: i32,
) -> k4a_wait_result_t;

pub(crate) type k4a_device_get_imu_sample = fn(
    device_handle: k4a_device_t,
    imu_sample: *mut k4a_imu_sample_t,
    timeout_in_ms: i32,
) -> k4a_wait_result_t;

pub(crate) type k4a_capture_create = fn(capture_handle: *mut k4a_capture_t) -> k4a_result_t;

pub(crate) type k4a_capture_release = fn(capture_handle: k4a_capture_t);

pub(crate) type k4a_capture_reference = fn(capture_handle: k4a_capture_t);

pub(crate) type k4a_capture_get_color_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;

pub(crate) type k4a_capture_get_depth_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;

pub(crate) type k4a_capture_get_ir_image = fn(capture_handle: k4a_capture_t) -> k4a_image_t;

pub(crate) type k4a_capture_set_color_image =
    fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);

pub(crate) type k4a_capture_set_depth_image =
    fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);

pub(crate) type k4a_capture_set_ir_image =
    fn(capture_handle: k4a_capture_t, image_handle: k4a_image_t);

pub(crate) type k4a_capture_set_temperature_c =
    fn(capture_handle: k4a_capture_t, temperature_c: f32);

pub(crate) type k4a_capture_get_temperature_c = fn(capture_handle: k4a_capture_t) -> f32;

pub(crate) type k4a_image_create = fn(
    format: k4a_image_format_t,
    width_pixels: i32,
    height_pixels: i32,
    stride_bytes: i32,
    image_handle: *mut k4a_image_t,
) -> k4a_result_t;

pub(crate) type k4a_image_create_from_buffer = fn(
    format: k4a_image_format_t,
    width_pixels: i32,
    height_pixels: i32,
    stride_bytes: i32,
    buffer: *mut u8,
    buffer_size: usize,
    buffer_release_cb: k4a_memory_destroy_cb_t,
    buffer_release_cb_context: *mut (),
    image_handle: *mut k4a_image_t,
) -> k4a_result_t;

pub(crate) type k4a_image_get_buffer = fn(image_handle: k4a_image_t) -> *mut u8;

pub(crate) type k4a_image_get_size = fn(image_handle: k4a_image_t) -> usize;

pub(crate) type k4a_image_get_format = fn(image_handle: k4a_image_t) -> k4a_image_format_t;

pub(crate) type k4a_image_get_width_pixels = fn(image_handle: k4a_image_t) -> i32;

pub(crate) type k4a_image_get_height_pixels = fn(image_handle: k4a_image_t) -> i32;

pub(crate) type k4a_image_get_stride_bytes = fn(image_handle: k4a_image_t) -> i32;

//pub(crate) type k4a_image_get_timestamp_usec = fn(image_handle: k4a_image_t) -> u64;

pub(crate) type k4a_image_get_device_timestamp_usec = fn(image_handle: k4a_image_t) -> u64;

pub(crate) type k4a_image_get_system_timestamp_nsec = fn(image_handle: k4a_image_t) -> u64;

pub(crate) type k4a_image_get_exposure_usec = fn(image_handle: k4a_image_t) -> u64;

pub(crate) type k4a_image_get_white_balance = fn(image_handle: k4a_image_t) -> u32;

pub(crate) type k4a_image_get_iso_speed = fn(image_handle: k4a_image_t) -> u32;

pub(crate) type k4a_image_set_device_timestamp_usec =
    fn(image_handle: k4a_image_t, timestamp_usec: u64);

//pub(crate) type k4a_image_set_timestamp_usec = fn(image_handle: k4a_image_t, timestamp_usec: u64);

pub(crate) type k4a_image_set_system_timestamp_nsec =
    fn(image_handle: k4a_image_t, timestamp_usec: u64);

pub(crate) type k4a_image_set_exposure_usec = fn(image_handle: k4a_image_t, timestamp_usec: u64);

//pub(crate) type k4a_image_set_exposure_time_usec =
//    fn(image_handle: k4a_image_t, timestamp_usec: u64);

pub(crate) type k4a_image_set_white_balance = fn(image_handle: k4a_image_t, white_balance: u32);

pub(crate) type k4a_image_set_iso_speed = fn(image_handle: k4a_image_t, white_balance: u32);

pub(crate) type k4a_image_reference = fn(image_handle: k4a_image_t);

pub(crate) type k4a_image_release = fn(image_handle: k4a_image_t);

pub(crate) type k4a_device_start_cameras =
    fn(device_handle: k4a_device_t, config: *const k4a_device_configuration_t) -> k4a_result_t;

pub(crate) type k4a_device_stop_cameras = fn(device_handle: k4a_device_t);

pub(crate) type k4a_device_start_imu = fn(device_handle: k4a_device_t) -> k4a_result_t;

pub(crate) type k4a_device_stop_imu = fn(device_handle: k4a_device_t);

pub(crate) type k4a_device_get_serialnum = fn(
    device_handle: k4a_device_t,
    serial_number: *mut ::std::os::raw::c_char,
    serial_number_size: *mut usize,
) -> k4a_buffer_result_t;

pub(crate) type k4a_device_get_version =
    fn(device_handle: k4a_device_t, version: *mut k4a_hardware_version_t) -> k4a_result_t;

pub(crate) type k4a_device_get_color_control_capabilities = fn(
    device_handle: k4a_device_t,
    command: k4a_color_control_command_t,
    supports_auto: *mut bool,
    min_value: *mut i32,
    max_value: *mut i32,
    step_value: *mut i32,
    default_value: *mut i32,
    default_mode: *mut k4a_color_control_mode_t,
) -> k4a_result_t;

pub(crate) type k4a_device_get_color_control = fn(
    device_handle: k4a_device_t,
    command: k4a_color_control_command_t,
    mode: *mut k4a_color_control_mode_t,
    value: *mut i32,
) -> k4a_result_t;

pub(crate) type k4a_device_set_color_control = fn(
    device_handle: k4a_device_t,
    command: k4a_color_control_command_t,
    mode: k4a_color_control_mode_t,
    value: i32,
) -> k4a_result_t;

pub(crate) type k4a_device_get_raw_calibration =
    fn(device_handle: k4a_device_t, data: *mut u8, data_size: *mut usize) -> k4a_buffer_result_t;

pub(crate) type k4a_device_get_calibration = fn(
    device_handle: k4a_device_t,
    depth_mode: k4a_depth_mode_t,
    color_resolution: k4a_color_resolution_t,
    calibration: *mut k4a_calibration_t,
) -> k4a_result_t;

pub(crate) type k4a_device_get_sync_jack = fn(
    device_handle: k4a_device_t,
    sync_in_jack_connected: *mut bool,
    sync_out_jack_connected: *mut bool,
) -> k4a_result_t;

pub(crate) type k4a_calibration_get_from_raw = fn(
    raw_calibration: *mut i8,
    raw_calibration_size: usize,
    depth_mode: k4a_depth_mode_t,
    color_resolution: k4a_color_resolution_t,
    calibration: *mut k4a_calibration_t,
) -> k4a_result_t;

pub(crate) type k4a_calibration_3d_to_3d = fn(
    calibration: *const k4a_calibration_t,
    source_point3d_mm: *const k4a_float3_t,
    source_camera: k4a_calibration_type_t,
    target_camera: k4a_calibration_type_t,
    target_point3d_mm: *mut k4a_float3_t,
) -> k4a_result_t;

pub(crate) type k4a_calibration_2d_to_3d = fn(
    calibration: *const k4a_calibration_t,
    source_point2d: *const k4a_float2_t,
    source_depth_mm: f32,
    source_camera: k4a_calibration_type_t,
    target_camera: k4a_calibration_type_t,
    target_point3d_mm: *mut k4a_float3_t,
    valid: *mut i32,
) -> k4a_result_t;

pub(crate) type k4a_calibration_3d_to_2d = fn(
    calibration: *const k4a_calibration_t,
    source_point3d_mm: *const k4a_float3_t,
    source_camera: k4a_calibration_type_t,
    target_camera: k4a_calibration_type_t,
    target_point2d: *mut k4a_float2_t,
    valid: *mut i32,
) -> k4a_result_t;

pub(crate) type k4a_calibration_2d_to_2d = fn(
    calibration: *const k4a_calibration_t,
    source_point2d: *const k4a_float2_t,
    source_depth_mm: f32,
    source_camera: k4a_calibration_type_t,
    target_camera: k4a_calibration_type_t,
    target_point2d: *mut k4a_float2_t,
    valid: *mut i32,
) -> k4a_result_t;

pub(crate) type k4a_calibration_color_2d_to_depth_2d = fn(
    calibration: *const k4a_calibration_t,
    source_point2d: *const k4a_float2_t,
    depth_image: k4a_image_t,
    target_point2d: *mut k4a_float2_t,
    valid: *mut i32,
) -> k4a_result_t;

pub(crate) type k4a_transformation_create =
    fn(calibration: *const k4a_calibration_t) -> k4a_transformation_t;

pub(crate) type k4a_transformation_destroy = fn(transformation_handle: k4a_transformation_t);

pub(crate) type k4a_transformation_depth_image_to_color_camera = fn(
    transformation_handle: k4a_transformation_t,
    depth_image: k4a_image_t,
    transformed_depth_image: k4a_image_t,
) -> k4a_result_t;

pub(crate) type k4a_transformation_depth_image_to_color_camera_custom = fn(
    transformation_handle: k4a_transformation_t,
    depth_image: k4a_image_t,
    custom_image: k4a_image_t,
    transformed_depth_image: k4a_image_t,
    transformed_custom_image: k4a_image_t,
    interpolation_type: k4a_transformation_interpolation_type_t,
    invalid_custom_value: u32,
) -> k4a_result_t;

pub(crate) type k4a_transformation_color_image_to_depth_camera = fn(
    transformation_handle: k4a_transformation_t,
    depth_image: k4a_image_t,
    custom_image: k4a_image_t,
    transformed_color_image: k4a_image_t,
) -> k4a_result_t;

pub(crate) type k4a_transformation_depth_image_to_point_cloud = fn(
    transformation_handle: k4a_transformation_t,
    depth_image: k4a_image_t,
    camera: k4a_calibration_type_t,
    xyz_image: k4a_image_t,
) -> k4a_result_t;

pub(crate) type k4a_playback_open =
    fn(path: *const ::std::os::raw::c_char, playback_handle: *mut k4a_playback_t) -> k4a_result_t;

pub(crate) type k4a_playback_get_raw_calibration = fn(
    playback_handle: k4a_playback_t,
    data: *mut u8,
    data_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_get_calibration =
    fn(playback_handle: k4a_playback_t, calibration: *mut k4a_calibration_t) -> k4a_result_t;

pub(crate) type k4a_playback_get_record_configuration =
    fn(playback_handle: k4a_playback_t, config: *mut k4a_record_configuration_t) -> k4a_result_t;

pub(crate) type k4a_playback_check_track_exists =
    fn(playback_handle: k4a_playback_t, track_name: *const ::std::os::raw::c_char) -> bool;

pub(crate) type k4a_playback_get_track_count = fn(playback_handle: k4a_playback_t) -> size_t;

pub(crate) type k4a_playback_get_track_name = fn(
    playback_handle: k4a_playback_t,
    track_index: size_t,
    track_name: *mut ::std::os::raw::c_char,
    track_name_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_track_is_builtin =
    fn(playback_handle: k4a_playback_t, track_name: *const ::std::os::raw::c_char) -> bool;

pub(crate) type k4a_playback_track_get_video_settings = fn(
    playback_handle: k4a_playback_t,
    track_name: *const ::std::os::raw::c_char,
    video_settings: *mut k4a_record_video_settings_t,
) -> k4a_result_t;

pub(crate) type k4a_playback_track_get_codec_id = fn(
    playback_handle: k4a_playback_t,
    track_name: *const ::std::os::raw::c_char,
    codec_id: *mut ::std::os::raw::c_char,
    codec_id_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_track_get_codec_context = fn(
    playback_handle: k4a_playback_t,
    track_name: *const ::std::os::raw::c_char,
    codec_context: *mut u8,
    codec_context_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_get_tag = fn(
    playback_handle: k4a_playback_t,
    name: *const ::std::os::raw::c_char,
    value: *mut ::std::os::raw::c_char,
    value_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_set_color_conversion =
    fn(playback_handle: k4a_playback_t, target_format: k4a_image_format_t) -> k4a_result_t;

pub(crate) type k4a_playback_get_attachment = fn(
    playback_handle: k4a_playback_t,
    file_name: *const ::std::os::raw::c_char,
    data: *mut u8,
    data_size: *mut size_t,
) -> k4a_buffer_result_t;

pub(crate) type k4a_playback_get_next_capture =
    fn(playback_handle: k4a_playback_t, capture_handle: *mut k4a_capture_t) -> k4a_stream_result_t;

pub(crate) type k4a_playback_get_previous_capture =
    fn(playback_handle: k4a_playback_t, capture_handle: *mut k4a_capture_t) -> k4a_stream_result_t;

pub(crate) type k4a_playback_get_next_imu_sample =
    fn(playback_handle: k4a_playback_t, imu_sample: *mut k4a_imu_sample_t) -> k4a_stream_result_t;

pub(crate) type k4a_playback_get_previous_imu_sample =
    fn(playback_handle: k4a_playback_t, imu_sample: *mut k4a_imu_sample_t) -> k4a_stream_result_t;

pub(crate) type k4a_playback_get_next_data_block = fn(
    playback_handle: k4a_playback_t,
    track_name: *const ::std::os::raw::c_char,
    data_block_handle: *mut k4a_playback_data_block_t,
) -> k4a_stream_result_t;

pub(crate) type k4a_playback_get_previous_data_block = fn(
    playback_handle: k4a_playback_t,
    track_name: *const ::std::os::raw::c_char,
    data_block_handle: *mut k4a_playback_data_block_t,
) -> k4a_stream_result_t;

pub(crate) type k4a_playback_data_block_get_device_timestamp_usec =
    fn(data_block_handle: k4a_playback_data_block_t) -> u64;

pub(crate) type k4a_playback_data_block_get_buffer_size =
    fn(data_block_handle: k4a_playback_data_block_t) -> size_t;

pub(crate) type k4a_playback_data_block_get_buffer =
    fn(data_block_handle: k4a_playback_data_block_t) -> *mut u8;

pub(crate) type k4a_playback_data_block_release = fn(data_block_handle: k4a_playback_data_block_t);

pub(crate) type k4a_playback_seek_timestamp = fn(
    playback_handle: k4a_playback_t,
    offset_usec: i64,
    origin: k4a_playback_seek_origin_t,
) -> k4a_result_t;

pub(crate) type k4a_playback_get_recording_length_usec = fn(playback_handle: k4a_playback_t) -> u64;

//pub(crate) type k4a_playback_get_last_timestamp_usec = fn(playback_handle: k4a_playback_t) -> u64;

pub(crate) type k4a_playback_close = fn(playback_handle: k4a_playback_t);

pub(crate) type k4a_record_create = fn(
    path: *const ::std::os::raw::c_char,
    device: k4a_device_t,
    device_config: k4a_device_configuration_t,
    recording_handle: *mut k4a_record_t,
) -> k4a_result_t;

pub(crate) type k4a_record_add_tag = fn(
    recording_handle: k4a_record_t,
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
) -> k4a_result_t;

pub(crate) type k4a_record_add_imu_track = fn(recording_handle: k4a_record_t) -> k4a_result_t;

pub(crate) type k4a_record_add_attachment = fn(
    recording_handle: k4a_record_t,
    attachment_name: *const ::std::os::raw::c_char,
    buffer: *const u8,
    buffer_size: size_t,
) -> k4a_result_t;

pub(crate) type k4a_record_add_custom_video_track = fn(
    recording_handle: k4a_record_t,
    track_name: *const ::std::os::raw::c_char,
    codec_id: *const ::std::os::raw::c_char,
    codec_context: *const u8,
    codec_context_size: size_t,
    track_settings: *const k4a_record_video_settings_t,
) -> k4a_result_t;

pub(crate) type k4a_record_add_custom_subtitle_track = fn(
    recording_handle: k4a_record_t,
    track_name: *const ::std::os::raw::c_char,
    codec_id: *const ::std::os::raw::c_char,
    codec_context: *const u8,
    codec_context_size: size_t,
    track_settings: *const k4a_record_subtitle_settings_t,
) -> k4a_result_t;

pub(crate) type k4a_record_write_header = fn(recording_handle: k4a_record_t) -> k4a_result_t;

pub(crate) type k4a_record_write_capture =
    fn(recording_handle: k4a_record_t, capture_handle: k4a_capture_t) -> k4a_result_t;

pub(crate) type k4a_record_write_imu_sample =
    fn(recording_handle: k4a_record_t, imu_sample: k4a_imu_sample_t) -> k4a_result_t;

pub(crate) type k4a_record_write_custom_track_data = fn(
    recording_handle: k4a_record_t,
    track_name: *const ::std::os::raw::c_char,
    device_timestamp_usec: u64,
    custom_data: *mut u8,
    custom_data_size: size_t,
) -> k4a_result_t;

pub(crate) type k4a_record_flush = fn(recording_handle: k4a_record_t) -> k4a_result_t;

pub(crate) type k4a_record_close = fn(recording_handle: k4a_record_t);
