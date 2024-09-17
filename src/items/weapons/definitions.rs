use crate::{
    currency::{cp, gp, sp},
    damage_types::DamageType,
    dice::{Dice, DiceAmount},
    items::weapons::{Weapon, WeaponGroup, WeaponTrait, WeaponType},
};

pub fn fists() -> Weapon {
    Weapon::new("Unarmed Attack")
        .price(cp(0))
        .dice(DiceAmount { n: 1, d: Dice::D4 })
        .damage_type(DamageType::Bludgeoning)
        .weapon_type(WeaponType::Unarmed)
        .weapon_group(WeaponGroup::Unarmed)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::Nonlethal,
        ])
        .build()
}

pub fn dagger() -> Weapon {
    Weapon::new("Dagger")
        .price(sp(2))
        .dice(DiceAmount { n: 1, d: Dice::D4 })
        .damage_type(DamageType::Piercing)
        .weapon_type(WeaponType::Simple)
        .weapon_group(WeaponGroup::Knife)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::VersatileSlashing,
        ])
        .build()
}

pub fn shortsword() -> Weapon {
    Weapon::new("Shortsword")
        .price(sp(9))
        .dice(DiceAmount { n: 1, d: Dice::D6 })
        .damage_type(DamageType::Piercing)
        .weapon_type(WeaponType::Martial)
        .weapon_group(WeaponGroup::Sword)
        .traits(vec![
            WeaponTrait::Agile,
            WeaponTrait::Finesse,
            WeaponTrait::VersatileSlashing,
        ])
        .build()
}

pub fn longsword() -> Weapon {
    Weapon::new("Longsword")
        .price(gp(1))
        .dice(DiceAmount { n: 1, d: Dice::D8 })
        .damage_type(DamageType::Slashing)
        .weapon_type(WeaponType::Martial)
        .weapon_group(WeaponGroup::Sword)
        .traits(vec![WeaponTrait::VersatilePiercing])
        .build()
}
