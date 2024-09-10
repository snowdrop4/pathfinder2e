use lazy_static::lazy_static;

use crate::{
    damage_types::DamageType,
    dice::Dice,
    items::weapon::{Weapon, WeaponGroup, WeaponTrait, WeaponType},
};

lazy_static! {
    pub static ref DAGGER: Weapon = Weapon::new("Dagger")
        .price(20)
        .dice(Dice::D4)
        .damage_type(vec![DamageType::Piercing])
        .weapon_type(WeaponType::Simple)
        .weapon_group(WeaponGroup::Knife)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::Versatile(DamageType::Slashing),
        ])
        .build();
    pub static ref SHORTSWORD: Weapon = Weapon::new("Shortsword")
        .price(900)
        .dice(Dice::D6)
        .damage_type(vec![DamageType::Piercing])
        .weapon_type(WeaponType::Martial)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::Versatile(DamageType::Slashing),
        ])
        .build();
    pub static ref LONGSWORD: Weapon = Weapon::new("Longsword")
        .price(1000)
        .dice(Dice::D8)
        .damage_type(vec![DamageType::Slashing])
        .weapon_type(WeaponType::Martial)
        .traits(vec![WeaponTrait::Versatile(DamageType::Piercing)])
        .build();
}
