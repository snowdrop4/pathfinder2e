pub mod definitions;
pub use definitions::*;

use crate::{
    currency::{cp, Currency},
    damage_types::DamageType,
    dice::{DamageAmount, Dice},
    player::Player,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Weapon {
    name: String,
    price: Currency,
    damage: DamageAmount,
    weapon_type: WeaponType,
    weapon_group: WeaponGroup,
    traits: Vec<WeaponTrait>,
    hands: u8,
    on_hit_effect: Option<fn(&Weapon, &mut Player)>,
}

impl Weapon {
    pub fn new(name: &str) -> WeaponBuilder {
        WeaponBuilder {
            name: name.to_string(),
            price: cp(1),
            damage: DamageAmount {
                n: 1,
                d: Dice::D4,
                t: DamageType::Slashing,
            },
            weapon_type: WeaponType::Simple,
            weapon_group: WeaponGroup::Sword,
            traits: Vec::new(),
            hands: 1,
            on_hit_effect: None,
        }
    }
}

pub struct WeaponBuilder {
    name: String,
    price: Currency,
    damage: DamageAmount,
    weapon_type: WeaponType,
    weapon_group: WeaponGroup,
    traits: Vec<WeaponTrait>,
    hands: u8,
    on_hit_effect: Option<fn(&Weapon, &mut Player)>,
}

impl WeaponBuilder {
    pub fn price(mut self, price: Currency) -> Self {
        self.price = price;
        self
    }

    pub fn damage(mut self, damage_amount: DamageAmount) -> Self {
        self.damage = damage_amount;
        self
    }

    pub fn weapon_type(mut self, weapon_type: WeaponType) -> Self {
        self.weapon_type = weapon_type;
        self
    }

    pub fn weapon_group(mut self, weapon_group: WeaponGroup) -> Self {
        self.weapon_group = weapon_group;
        self
    }

    pub fn traits(mut self, traits: Vec<WeaponTrait>) -> Self {
        self.traits = traits;
        self
    }

    pub fn hands(mut self, hands: u8) -> Self {
        self.hands = hands;
        self
    }

    pub fn on_hit_effect(mut self, effect: Option<fn(&Weapon, &mut Player)>) -> Self {
        self.on_hit_effect = effect;
        self
    }

    pub fn build(self) -> Weapon {
        Weapon {
            name: self.name,
            price: self.price,
            damage: self.damage,
            weapon_type: self.weapon_type,
            weapon_group: self.weapon_group,
            traits: self.traits,
            hands: self.hands,
            on_hit_effect: self.on_hit_effect,
        }
    }
}

#[derive(Debug)]
pub enum WeaponType {
    Simple,
    Martial,
    Advanced,
}

#[derive(Debug)]
pub enum WeaponGroup {
    Sword,
    Knife,
}

#[derive(Debug)]
pub enum WeaponTrait {
    Agile,
    Finesse,
    Versatile(DamageType),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dagger() {
        let w = dagger();

        for _ in 0..100 {
            let damage = w.damage.sum();
            assert!(damage >= 1 && damage <= 4)
        }
    }

    #[test]
    fn test_shortsword() {
        let w = shortsword();

        for _ in 0..100 {
            let damage = w.damage.sum();
            assert!(damage >= 1 && damage <= 4)
        }
    }

    #[test]
    fn test_longsword() {
        let w = longsword();

        for _ in 0..100 {
            let damage = w.damage.sum();
            assert!(damage >= 1 && damage <= 4)
        }
    }
}
