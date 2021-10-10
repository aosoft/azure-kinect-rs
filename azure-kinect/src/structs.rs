use azure_kinect_sys::k4a::k4a_device_configuration_t;
use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

#[derive(Copy, Clone, Default)]
pub struct DeviceConfiguration {
    pub(crate) value: k4a_device_configuration_t,
}

impl DeviceConfiguration {
    pub fn builder() -> DeviceConfigurationBuilder { DeviceConfigurationBuilder::default() }

    #[doc = " Image format to capture with the color camera."]
    pub fn color_format(&self) -> ImageFormat { ImageFormat::from_primitive(self.value.color_format) }

    #[doc = " Image resolution to capture with the color camera."]
    pub fn color_resolution(&self) -> ColorResolution { ColorResolution::from_primitive(self.value.color_resolution) }

    #[doc = " Capture mode for the depth camera."]
    pub fn depth_mode(&self) -> DepthMode { DepthMode::from_primitive(self.value.depth_mode) }

    #[doc = " Desired frame rate for the color and depth camera."]
    pub fn camera_fps(&self) -> Fps { Fps::from_primitive(self.value.camera_fps.into()) }

    #[doc = " Only produce k4a_capture_t objects if they contain synchronized color and depth images."]
    pub fn synchronized_images_only(&self) -> bool { self.value.synchronized_images_only }

    #[doc = " Desired delay between the capture of the color image and the capture of the depth image."]
    pub fn depth_delay_off_color_usec(&self) -> i32 { self.value.depth_delay_off_color_usec }

    #[doc = " The external synchronization mode."]
    pub fn wired_sync_mode(&self) -> WiredSyncMode { WiredSyncMode::from_primitive(self.value.wired_sync_mode.into()) }

    #[doc = " The external synchronization timing."]
    pub fn subordinate_delay_off_master_usec(&self) -> u32 { self.value.subordinate_delay_off_master_usec }

    #[doc = " Streaming indicator automatically turns on when the color or depth camera's are in use."]
    pub fn disable_streaming_indicator(&self) -> bool { self.value.disable_streaming_indicator }
}

#[derive(Default)]
pub struct DeviceConfigurationBuilder {
    value: k4a_device_configuration_t,
}

impl DeviceConfigurationBuilder {
    #[doc = " Image format to capture with the color camera."]
    pub fn color_format(mut self, value: ImageFormat) -> DeviceConfigurationBuilder {
        self.value.color_format = value.into();
        self
    }

    #[doc = " Image resolution to capture with the color camera."]
    pub fn color_resolution(mut self, value: ColorResolution) -> DeviceConfigurationBuilder {
        self.value.color_resolution = value.into();
        self
    }

    #[doc = " Capture mode for the depth camera."]
    pub fn depth_mode(mut self, value: DepthMode) -> DeviceConfigurationBuilder {
        self.value.depth_mode = value.into();
        self
    }

    #[doc = " Desired frame rate for the color and depth camera."]
    pub fn camera_fps(mut self, value: Fps) -> DeviceConfigurationBuilder {
        self.value.camera_fps = value.into();
        self
    }

    #[doc = " Only produce k4a_capture_t objects if they contain synchronized color and depth images."]
    pub fn synchronized_images_only(mut self, value: bool) -> DeviceConfigurationBuilder {
        self.value.synchronized_images_only = value;
        self
    }

    #[doc = " Desired delay between the capture of the color image and the capture of the depth image."]
    pub fn depth_delay_off_color_usec(mut self, value: i32) -> DeviceConfigurationBuilder {
        self.value.depth_delay_off_color_usec = value;
        self
    }

    #[doc = " The external synchronization mode."]
    pub fn wired_sync_mode(mut self, value: WiredSyncMode) -> DeviceConfigurationBuilder {
        self.value.wired_sync_mode = value.into();
        self
    }

    #[doc = " The external synchronization timing."]
    pub fn subordinate_delay_off_master_usec(mut self, value: u32) -> DeviceConfigurationBuilder {
        self.value.subordinate_delay_off_master_usec = value;
        self
    }

    #[doc = " Streaming indicator automatically turns on when the color or depth camera's are in use."]
    pub fn disable_streaming_indicator(mut self, value: bool) -> DeviceConfigurationBuilder {
        self.value.disable_streaming_indicator = value;
        self
    }

    pub fn build(&self) -> DeviceConfiguration {
        DeviceConfiguration { value: self.value }
    }
}