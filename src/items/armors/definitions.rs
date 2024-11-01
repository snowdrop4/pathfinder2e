use super::Armor;
use crate::currency::cp;

pub fn skin() -> Armor {
    Armor::new("Skin")
        .price(cp(0))
        .armor_type(super::ArmorType::Unarmored)
        .ac_bonus(0)
        .dex_cap(None)
        .strength_required(None)
        .check_penalty(0)
        .speed_penalty(0)
        .traits(Vec::new())
        .build()
}
