pub mod human;

use crate::attribute::Attribute;
use crate::size::Size;

#[derive(Debug)]
pub struct Ancestry {
    pub hp: i64,
    pub size: Size,
    pub stride_speed: i64,
    pub climb_speed: Option<i64>,
    pub swim_speed: Option<i64>,
    pub fly_speed: Option<i64>,
    pub attribute_boosts: (Attribute, Attribute),
}
