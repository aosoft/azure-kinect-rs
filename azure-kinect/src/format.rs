use super::*;

pub struct Dimension {
    pub width: i32,
    pub height: i32,
}

pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl k4a_color_resolution_t {
    /// Gets the dimensions of the color images that the color camera will produce for a
    /// given color resolution
    pub fn get_dimension(&self) -> Dimension {
        match self {
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_720P => Dimension {
                width: 1280,
                height: 720,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1080P => Dimension {
                width: 1920,
                height: 1080,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1440P => Dimension {
                width: 2560,
                height: 1440,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1536P => Dimension {
                width: 2048,
                height: 1536,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_2160P => Dimension {
                width: 3840,
                height: 2160,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_3072P => Dimension {
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

impl k4a_depth_mode_t {
    /// Gets the dimensions of the depth images that the depth camera will produce for a
    /// given depth mode
    pub fn get_dimension(&self) -> Dimension {
        match self {
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_2X2BINNED => Dimension {
                width: 320,
                height: 288,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_UNBINNED => Dimension {
                width: 640,
                height: 576,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_2X2BINNED => Dimension {
                width: 512,
                height: 512,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_UNBINNED => Dimension {
                width: 1024,
                height: 1024,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_PASSIVE_IR => Dimension {
                width: 1024,
                height: 1024,
            },
            _ => Dimension {
                width: 0,
                height: 0,
            },
        }
    }

    /// Gets the range of values that we expect to see from the depth camera
    /// when using a given depth mode, in millimeters
    pub fn get_range(&self) -> Range<u16> {
        match self {
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_2X2BINNED => Range::<u16> {
                min: 500,
                max: 5800,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_UNBINNED => Range::<u16> {
                min: 500,
                max: 4000,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_2X2BINNED => Range::<u16> {
                min: 250,
                max: 3000,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_UNBINNED => Range::<u16> {
                min: 250,
                max: 2500,
            },
            _ => Range::<u16> { min: 0, max: 0 },
        }
    }

    /// Gets the expected min/max IR brightness levels that we expect to see
    /// from the IR camera when using a given depth mode
    pub fn get_ir_level(&self) -> Range<u16> {
        match self {
            k4a_depth_mode_t::K4A_DEPTH_MODE_PASSIVE_IR => Range::<u16> {
                min: 250,
                max: 3000,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_OFF => Range::<u16> { min: 0, max: 0 },
            _ => Range::<u16> { min: 0, max: 1000 },
        }
    }
}
