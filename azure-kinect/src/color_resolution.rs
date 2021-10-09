use crate::Dimension;
use azure_kinect_sys::k4a::*;

#[repr(u32)]
pub enum ColorResolution {
    Off = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
    _720p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
    _1080p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1080P,
    _1440p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1440P,
    _1536p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_1536P,
    _2160p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_2160P,
    _3072p = k4a_color_resolution_t_K4A_COLOR_RESOLUTION_3072P,
}

impl ColorResolution {
    /// Gets the dimensions of the color images that the color camera will produce for a
    /// given color resolution
    pub fn get_dimension(&self) -> Dimension {
        match self {
            ColorResolution::_720p => Dimension {
                width: 1280,
                height: 720,
            },
            ColorResolution::_1080p => Dimension {
                width: 1920,
                height: 1080,
            },
            ColorResolution::_1440p => Dimension {
                width: 2560,
                height: 1440,
            },
            ColorResolution::_1536p => Dimension {
                width: 2048,
                height: 1536,
            },
            ColorResolution::_2160p => Dimension {
                width: 3840,
                height: 2160,
            },
            ColorResolution::_3072p => Dimension {
                width: 4096,
                height: 3072,
            },
            _ => Dimension {
                width: 0,
                height: 0,
            },
        }
    }
}
