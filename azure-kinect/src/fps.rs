use azure_kinect_sys::k4a::*;

#[repr(u32)]
pub enum Fps {
    _5fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_5,
    _15fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_15,
    _30fps = k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
}

impl Fps {
    pub fn get_u32(&self) -> u32 {
        match self {
            Fps::_5fps => 5,
            Fps::_15fps => 15,
            Fps::_30fps => 30,
        }
    }
}
