use crate::{attribute::Attribute, size::Size};

use super::Ancestry;

impl Ancestry {
    pub fn new_human(attribute_boosts: (Attribute, Attribute)) -> Ancestry {
        return Ancestry {
            hp: 8,
            size: Size::Medium,
            stride_speed: 25,
            attribute_boosts,
        };
    }
}
