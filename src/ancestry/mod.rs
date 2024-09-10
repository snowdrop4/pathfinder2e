use crate::{attribute::Attribute, size::Size};
mod human;

#[derive(Debug)]
pub struct Ancestry {
    pub hp: i64,
    pub size: Size,
    pub speed: i64,
    pub attribute_boosts: (Attribute, Attribute),
}
