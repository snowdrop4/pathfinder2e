use crate::{
    currency::{gp, sp},
    damage_types::DamageType,
    dice::{DamageAmount, Dice},
    items::weapons::{Weapon, WeaponGroup, WeaponTrait, WeaponType},
};

pub fn dagger() -> Weapon {
    Weapon::new("Dagger")
        .price(sp(2))
        .damage(DamageAmount {
            n: 1,
            d: Dice::D4,
            t: DamageType::Piercing,
        })
        .weapon_type(WeaponType::Simple)
        .weapon_group(WeaponGroup::Knife)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::Versatile(DamageType::Slashing),
        ])
        .build()
}

pub fn shortsword() -> Weapon {
    Weapon::new("Shortsword")
        .price(sp(9))
        .damage(DamageAmount {
            n: 1,
            d: Dice::D6,
            t: DamageType::Piercing,
        })
        .weapon_type(WeaponType::Martial)
        .weapon_group(WeaponGroup::Sword)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::Versatile(DamageType::Slashing),
        ])
        .build()
}

pub fn longsword() -> Weapon {
    Weapon::new("Longsword")
        .price(gp(1))
        .damage(DamageAmount {
            n: 1,
            d: Dice::D8,
            t: DamageType::Slashing,
        })
        .weapon_type(WeaponType::Martial)
        .weapon_group(WeaponGroup::Sword)
        .traits(vec![WeaponTrait::Versatile(DamageType::Piercing)])
        .build()
}
