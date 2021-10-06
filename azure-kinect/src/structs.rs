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



