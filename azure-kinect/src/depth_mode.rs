use azure_kinect_sys::k4a::*;
use crate::{Dimension, Range};

#[repr(u32)]
pub enum DepthMode {
    Off = k4a_depth_mode_t_K4A_DEPTH_MODE_OFF,
    NFov2x2Binned = k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_2X2BINNED,
    NFovUnbinned = k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED,
    WFov2x2Binned = k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED,
    WFovUnbinned = k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_UNBINNED,
    PassiveIr = k4a_depth_mode_t_K4A_DEPTH_MODE_PASSIVE_IR,
}

impl DepthMode {
    /// Gets the dimensions of the depth images that the depth camera will produce for a
    /// given depth mode
    pub fn get_dimension(&self) -> Dimension {
        match self {
            NFov2x2Binned => Dimension {
                width: 320,
                height: 288,
            },
            NFovUnbinned => Dimension {
                width: 640,
                height: 576,
            },
            WFov2x2Binned => Dimension {
                width: 512,
                height: 512,
            },
            WFovUnbinned => Dimension {
                width: 1024,
                height: 1024,
            },
            PassiveIr => Dimension {
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
            NFov2x2Binned => Range::<u16> {
                min: 500,
                max: 5800,
            },
            NFovUnbinned => Range::<u16> {
                min: 500,
                max: 4000,
            },
            WFov2x2Binned => Range::<u16> {
                min: 250,
                max: 3000,
            },
            WFovUnbinned => Range::<u16> {
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
            PassiveIr => Range::<u16> {
                min: 250,
                max: 3000,
            },
            Off => Range::<u16> { min: 0, max: 0 },
            _ => Range::<u16> { min: 0, max: 1000 },
        }
    }
}
