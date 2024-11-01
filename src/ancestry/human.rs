use super::Ancestry;
use crate::attribute::Attribute;
use crate::size::Size;

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
