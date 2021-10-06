use super::Error;
use super::loader::Module;

const K4A_LIBNAME: &'static str = "k4a.dll";
const K4ARECORD_LIBNAME: &'static str = "k4arecord.dll";

macro_rules! proc_address {
    ($m:ident, $proc_name:ident) => {
        std::mem::transmute(
            $m.get_proc_address(concat!(stringify!($proc_name), "\0").as_ptr())?,
        )
    };
}

pub struct Api {
    module: Module,
    k4a: super::k4a::Funcs,
}

impl Api {
    pub(crate) fn with_module(module: Module) -> Result<Api, Error> {
        unsafe {
            let funcs = super::k4a::Funcs {
                k4a_device_get_installed_count: proc_address!(module, k4a_device_get_installed_count),
                k4a_set_debug_message_handler: proc_address!(module, k4a_set_debug_message_handler),
                k4a_set_allocator: proc_address!(module, k4a_set_allocator),
                k4a_device_open: proc_address!(module, k4a_device_open),
                k4a_device_close: proc_address!(module, k4a_device_close),
                k4a_device_get_capture: proc_address!(module, k4a_device_get_capture),
                k4a_device_get_imu_sample: proc_address!(module, k4a_device_get_imu_sample),
                k4a_capture_create: proc_address!(module, k4a_capture_create),
                k4a_capture_release: proc_address!(module, k4a_capture_release),
                k4a_capture_reference: proc_address!(module, k4a_capture_reference),
                k4a_capture_get_color_image: proc_address!(module, k4a_capture_get_color_image),
                k4a_capture_get_depth_image: proc_address!(module, k4a_capture_get_depth_image),
                k4a_capture_get_ir_image: proc_address!(module, k4a_capture_get_ir_image),
                k4a_capture_set_color_image: proc_address!(module, k4a_capture_set_color_image),
                k4a_capture_set_depth_image: proc_address!(module, k4a_capture_set_depth_image),
                k4a_capture_set_ir_image: proc_address!(module, k4a_capture_set_ir_image),
                k4a_capture_set_temperature_c: proc_address!(module, k4a_capture_set_temperature_c),
                k4a_capture_get_temperature_c: proc_address!(module, k4a_capture_get_temperature_c),
                k4a_image_create: proc_address!(module, k4a_image_create),
                k4a_image_create_from_buffer: proc_address!(module, k4a_image_create_from_buffer),
                k4a_image_get_buffer: proc_address!(module, k4a_image_get_buffer),
                k4a_image_get_size: proc_address!(module, k4a_image_get_size),
                k4a_image_get_format: proc_address!(module, k4a_image_get_format),
                k4a_image_get_width_pixels: proc_address!(module, k4a_image_get_width_pixels),
                k4a_image_get_height_pixels: proc_address!(module, k4a_image_get_height_pixels),
                k4a_image_get_stride_bytes: proc_address!(module, k4a_image_get_stride_bytes),
                k4a_image_get_timestamp_usec: proc_address!(module, k4a_image_get_timestamp_usec),
                k4a_image_get_device_timestamp_usec: proc_address!(module, k4a_image_get_device_timestamp_usec),
                k4a_image_get_system_timestamp_nsec: proc_address!(module, k4a_image_get_system_timestamp_nsec),
                k4a_image_get_exposure_usec: proc_address!(module, k4a_image_get_exposure_usec),
                k4a_image_get_white_balance: proc_address!(module, k4a_image_get_white_balance),
                k4a_image_get_iso_speed: proc_address!(module, k4a_image_get_iso_speed),
                k4a_image_set_device_timestamp_usec: proc_address!(module, k4a_image_set_device_timestamp_usec),
                k4a_image_set_timestamp_usec: proc_address!(module, k4a_image_set_timestamp_usec),
                k4a_image_set_system_timestamp_nsec: proc_address!(module, k4a_image_set_system_timestamp_nsec),
                k4a_image_set_exposure_usec: proc_address!(module, k4a_image_set_exposure_usec),
                k4a_image_set_exposure_time_usec: proc_address!(module, k4a_image_set_exposure_time_usec),
                k4a_image_set_white_balance: proc_address!(module, k4a_image_set_white_balance),
                k4a_image_set_iso_speed: proc_address!(module, k4a_image_set_iso_speed),
                k4a_image_reference: proc_address!(module, k4a_image_reference),
                k4a_image_release: proc_address!(module, k4a_image_release),
                k4a_device_start_cameras: proc_address!(module, k4a_device_start_cameras),
                k4a_device_stop_cameras: proc_address!(module, k4a_device_stop_cameras),
                k4a_device_start_imu: proc_address!(module, k4a_device_start_imu),
                k4a_device_stop_imu: proc_address!(module, k4a_device_stop_imu),
                k4a_device_get_serialnum: proc_address!(module, k4a_device_get_serialnum),
                k4a_device_get_version: proc_address!(module, k4a_device_get_version),
                k4a_device_get_color_control_capabilities: proc_address!(module, k4a_device_get_color_control_capabilities),
                k4a_device_get_color_control: proc_address!(module, k4a_device_get_color_control),
                k4a_device_set_color_control: proc_address!(module, k4a_device_set_color_control),
                k4a_device_get_raw_calibration: proc_address!(module, k4a_device_get_raw_calibration),
                k4a_device_get_calibration: proc_address!(module, k4a_device_get_calibration),
                k4a_device_get_sync_jack: proc_address!(module, k4a_device_get_sync_jack),
                k4a_calibration_get_from_raw: proc_address!(module, k4a_calibration_get_from_raw),
                k4a_calibration_3d_to_3d: proc_address!(module, k4a_calibration_3d_to_3d),
                k4a_calibration_2d_to_3d: proc_address!(module, k4a_calibration_2d_to_3d),
                k4a_calibration_3d_to_2d: proc_address!(module, k4a_calibration_3d_to_2d),
                k4a_calibration_2d_to_2d: proc_address!(module, k4a_calibration_2d_to_2d),
                k4a_calibration_color_2d_to_depth_2d: proc_address!(module, k4a_calibration_color_2d_to_depth_2d),
                k4a_transformation_create: proc_address!(module, k4a_transformation_create),
                k4a_transformation_destroy: proc_address!(module, k4a_transformation_destroy),
                k4a_transformation_depth_image_to_color_camera: proc_address!(module, k4a_transformation_depth_image_to_color_camera),
                k4a_transformation_depth_image_to_color_camera_custom: proc_address!(module, k4a_transformation_depth_image_to_color_camera_custom),
                k4a_transformation_color_image_to_depth_camera: proc_address!(module, k4a_transformation_color_image_to_depth_camera),
                k4a_transformation_depth_image_to_point_cloud: proc_address!(module, k4a_transformation_depth_image_to_point_cloud),
            };

            Ok(Api {
                module: module,
                k4a: funcs,
            })
        }
    }

    pub fn new() -> Result<Api, Error> {
        Self::with_library_directory(
            std::env::current_exe()
                .map_err(|_| Error::Failed)?
                .parent()
                .ok_or(Error::Failed)?
                .to_str()
                .ok_or(Error::Failed)?,
        )
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<Api, Error> {
        Self::with_module(Module::load_library(lib_dir, K4A_LIBNAME)?)
    }

    pub fn k4a(&self) -> &super::k4a::Funcs { &self.k4a }
}

pub struct ApiRecord {
    module: Module,
    k4a: Api,
    k4arecord: super::k4arecord::Funcs,
}

impl ApiRecord {
    pub(crate) fn with_module(module: Module) -> Result<ApiRecord, Error> {
        unsafe {
            let funcs = super::k4arecord::Funcs {
                k4a_record_create: proc_address!(module, k4a_record_create),
                k4a_record_add_tag: proc_address!(module, k4a_record_add_tag),
                k4a_record_add_imu_track: proc_address!(module, k4a_record_add_imu_track),
                k4a_record_add_attachment: proc_address!(module, k4a_record_add_attachment),
                k4a_record_add_custom_video_track: proc_address!(module, k4a_record_add_custom_video_track),
                k4a_record_add_custom_subtitle_track: proc_address!(module, k4a_record_add_custom_subtitle_track),
                k4a_record_write_header: proc_address!(module, k4a_record_write_header),
                k4a_record_write_capture: proc_address!(module, k4a_record_write_capture),
                k4a_record_write_imu_sample: proc_address!(module, k4a_record_write_imu_sample),
                k4a_record_write_custom_track_data: proc_address!(module, k4a_record_write_custom_track_data),
                k4a_record_flush: proc_address!(module, k4a_record_flush),
                k4a_record_close: proc_address!(module, k4a_record_close),
                k4a_playback_open: proc_address!(module, k4a_playback_open),
                k4a_playback_get_raw_calibration: proc_address!(module, k4a_playback_get_raw_calibration),
                k4a_playback_get_calibration: proc_address!(module, k4a_playback_get_calibration),
                k4a_playback_get_record_configuration: proc_address!(module, k4a_playback_get_record_configuration),
                k4a_playback_check_track_exists: proc_address!(module, k4a_playback_check_track_exists),
                k4a_playback_get_track_count: proc_address!(module, k4a_playback_get_track_count),
                k4a_playback_get_track_name: proc_address!(module, k4a_playback_get_track_name),
                k4a_playback_track_is_builtin: proc_address!(module, k4a_playback_track_is_builtin),
                k4a_playback_track_get_video_settings: proc_address!(module, k4a_playback_track_get_video_settings),
                k4a_playback_track_get_codec_id: proc_address!(module, k4a_playback_track_get_codec_id),
                k4a_playback_track_get_codec_context: proc_address!(module, k4a_playback_track_get_codec_context),
                k4a_playback_get_tag: proc_address!(module, k4a_playback_get_tag),
                k4a_playback_set_color_conversion: proc_address!(module, k4a_playback_set_color_conversion),
                k4a_playback_get_attachment: proc_address!(module, k4a_playback_get_attachment),
                k4a_playback_get_next_capture: proc_address!(module, k4a_playback_get_next_capture),
                k4a_playback_get_previous_capture: proc_address!(module, k4a_playback_get_previous_capture),
                k4a_playback_get_next_imu_sample: proc_address!(module, k4a_playback_get_next_imu_sample),
                k4a_playback_get_previous_imu_sample: proc_address!(module, k4a_playback_get_previous_imu_sample),
                k4a_playback_get_next_data_block: proc_address!(module, k4a_playback_get_next_data_block),
                k4a_playback_get_previous_data_block: proc_address!(module, k4a_playback_get_previous_data_block),
                k4a_playback_data_block_get_device_timestamp_usec: proc_address!(module, k4a_playback_data_block_get_device_timestamp_usec),
                k4a_playback_data_block_get_buffer_size: proc_address!(module, k4a_playback_data_block_get_buffer_size),
                k4a_playback_data_block_get_buffer: proc_address!(module, k4a_playback_data_block_get_buffer),
                k4a_playback_data_block_release: proc_address!(module, k4a_playback_data_block_release),
                k4a_playback_seek_timestamp: proc_address!(module, k4a_playback_seek_timestamp),
                k4a_playback_get_recording_length_usec: proc_address!(module, k4a_playback_get_recording_length_usec),
                k4a_playback_get_last_timestamp_usec: proc_address!(module, k4a_playback_get_last_timestamp_usec),
                k4a_playback_close: proc_address!(module, k4a_playback_close),
            };

            Ok(ApiRecord {
                module: module,
                k4a: Api::with_module(Module::get_module(K4A_LIBNAME)?)?,
                k4arecord: funcs,
            })
        }
    }

    pub fn new() -> Result<ApiRecord, Error> {
        Self::with_library_directory(
            std::env::current_exe()
                .map_err(|_| Error::Failed)?
                .parent()
                .ok_or(Error::Failed)?
                .to_str()
                .ok_or(Error::Failed)?,
        )
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<ApiRecord, Error> {
        Self::with_module(Module::load_library(lib_dir, K4ARECORD_LIBNAME)?)
    }

    pub fn k4a(&self) -> &super::k4a::Funcs { &self.k4a.k4a }
    pub fn k4arecord(&self) -> &super::k4arecord::Funcs { &self.k4arecord }
}