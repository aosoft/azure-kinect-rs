use super::factory::Factory;
use super::bindings::*;

pub struct Device<'a> {
    pub(crate) factory: &'a Factory,
    pub(crate) handle: k4a_device_t
}