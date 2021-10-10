use azure_kinect_sys::k4a::*;
use crate::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Version {
    pub(crate) value: k4a_version_t,
}

impl Version {
    #[doc = "< Major version; represents a breaking change."]
    pub fn major(&self) -> u32 { self.value.major }
    #[doc = "< Minor version; represents additional features, no regression from lower versions with same major version."]
    pub fn minor(&self) -> u32 { self.value.minor }
    #[doc = "< Reserved."]
    pub fn iteration(&self) -> u32 { self.value.iteration }
}


#[derive(Copy, Clone, Debug, Default)]
pub struct HardwareVersion {
    pub(crate) value: k4a_hardware_version_t,
}

impl HardwareVersion {
    #[doc = "< Color camera firmware version."]
    pub fn rgb(&self) -> Version { Version { value: self.value.rgb }}
    #[doc = "< Depth camera firmware version."]
    pub fn depth(&self) -> Version { Version { value: self.value.depth }}
    #[doc = "< Audio device firmware version."]
    pub fn audio(&self) -> Version { Version { value: self.value.audio }}
    #[doc = "< Depth sensor firmware version."]
    pub fn depth_sensor(&self) -> Version { Version { value: self.value.depth_sensor }}
    #[doc = "< Build type reported by the firmware."]
    pub fn firmware_build(&self) -> FirmwareBuildType { FirmwareBuildType::from_primitive(self.value.firmware_build) }
    #[doc = "< Signature type of the firmware."]
    pub fn firmware_signature(&self) -> FirmwareSignatureType { FirmwareSignatureType::from_primitive(self.value.firmware_signature) }
}

