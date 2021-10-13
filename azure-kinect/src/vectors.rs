use azure_kinect_sys::k4a::*;
use std::fmt::{Display, Formatter};

#[doc = " Two dimensional floating point vector."]
#[derive(Copy, Clone, Default)]
pub struct Float2 {
    pub(crate) value: k4a_float2_t,
}

impl Float2 {
    pub fn new(x: f32, y: f32) -> Float2 {
        Float2 {
            value: k4a_float2_t {
                xy: k4a_float2_t__xy { x, y },
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn from_native(value: k4a_float2_t) -> Float2 {
        Float2 { value }
    }

    #[doc = "< X component of a vector"]
    pub fn x(&self) -> f32 {
        unsafe { self.value.xy.x }
    }
    #[doc = "< Y component of a vector"]
    pub fn y(&self) -> f32 {
        unsafe { self.value.xy.y }
    }
}

impl Display for Float2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}

#[doc = " Three dimensional floating point vector."]
#[derive(Copy, Clone, Default)]
pub struct Float3 {
    pub(crate) value: k4a_float3_t,
}

impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Float3 {
        Float3 {
            value: k4a_float3_t {
                xyz: k4a_float3_t__xyz { x, y, z },
            },
        }
    }

    pub(crate) fn from_native(value: k4a_float3_t) -> Float3 {
        Float3 { value }
    }

    #[doc = "< X component of a vector"]
    pub fn x(&self) -> f32 {
        unsafe { self.value.xyz.x }
    }
    #[doc = "< Y component of a vector"]
    pub fn y(&self) -> f32 {
        unsafe { self.value.xyz.y }
    }
    #[doc = "< Z component of a vector"]
    pub fn z(&self) -> f32 {
        unsafe { self.value.xyz.z }
    }
}

impl Display for Float3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
