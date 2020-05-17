use super::*;

pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

impl k4a_color_resolution_t {
    pub fn get_resolution(&self) -> Resolution {
        match self {
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_720P => Resolution {
                width: 1280,
                height: 720,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1080P => Resolution {
                width: 1920,
                height: 1080,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1440P => Resolution {
                width: 2560,
                height: 1440,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_1536P => Resolution {
                width: 2048,
                height: 1536,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_2160P => Resolution {
                width: 3840,
                height: 2160,
            },
            k4a_color_resolution_t::K4A_COLOR_RESOLUTION_3072P => Resolution {
                width: 4096,
                height: 3072,
            },
            _ => Resolution {
                width: 0,
                height: 0,
            },
        }
    }
}

impl k4a_depth_mode_t {
    pub fn get_resolution(&self) -> Resolution {
        match self {
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_2X2BINNED => Resolution {
                width: 320,
                height: 288,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_NFOV_UNBINNED => Resolution {
                width: 640,
                height: 576,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_2X2BINNED => Resolution {
                width: 512,
                height: 512,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_WFOV_UNBINNED => Resolution {
                width: 1024,
                height: 1024,
            },
            k4a_depth_mode_t::K4A_DEPTH_MODE_PASSIVE_IR => Resolution {
                width: 1024,
                height: 1024,
            },
            _ => Resolution {
                width: 0,
                height: 0,
            },
        }
    }
}
