pub mod definitions;
pub use definitions::*;

use crate::attribute::Attribute;
use crate::attributes::Attributes;
use crate::currency::{cp, Currency};
use crate::damage_types::DamageType;
use crate::dice::{Dice, DiceAmount};
use crate::player::Player;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Weapon {
    pub name: String,
    pub price: Currency,
    pub dice: DiceAmount,
    pub damage_type: DamageType,
    pub weapon_type: WeaponType,
    pub weapon_group: WeaponGroup,
    pub traits: Vec<WeaponTrait>,
    pub hands: u8,
    pub on_hit_effect: Option<fn(&Weapon, &mut Player)>,
}

impl Weapon {
    pub fn new(name: &str) -> WeaponBuilder {
        WeaponBuilder {
            name: name.to_string(),
            price: cp(1),
            dice: DiceAmount { n: 1, d: Dice::D4 },
            damage_type: DamageType::Slashing,
            weapon_type: WeaponType::Simple,
            weapon_group: WeaponGroup::Sword,
            traits: Vec::new(),
            hands: 1,
            on_hit_effect: None,
        }
    }

    pub fn get_multiple_attack_penalty(&self, attack_index: i64) -> i64 {
        if self.traits.contains(&WeaponTrait::Agile) {
            return attack_index * -4;
        } else {
            return attack_index * -5;
        }
    }

    pub fn get_attribute_damage_mod(&self, attributes: &Attributes) -> i64 {
        attributes.get(Attribute::Strength)
    }

    pub fn get_attribute_attack_mod(&self, attributes: &Attributes) -> i64 {
        if self.traits.contains(&WeaponTrait::Finesse) {
            let dex = attributes.get(Attribute::Dexterity);
            let str = attributes.get(Attribute::Strength);

            return std::cmp::max(dex, str);
        } else {
            return attributes.get(Attribute::Strength);
        }
    }

    pub fn get_damage_types(&self) -> Vec<DamageType> {
        let mut v = vec![self.damage_type];

        if self.traits.contains(&WeaponTrait::VersatilePiercing) {
            v.push(DamageType::Piercing);
        }

        if self.traits.contains(&WeaponTrait::VersatileSlashing) {
            v.push(DamageType::Slashing);
        }

        if self.traits.contains(&WeaponTrait::VersatileBludgeoning) {
            v.push(DamageType::Bludgeoning);
        }

        v
    }
}

pub struct WeaponBuilder {
    name: String,
    price: Currency,
    dice: DiceAmount,
    damage_type: DamageType,
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

    pub fn dice(mut self, dice: DiceAmount) -> Self {
        self.dice = dice;
        self
    }

    pub fn damage_type(mut self, damage_type: DamageType) -> Self {
        self.damage_type = damage_type;
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
            dice: self.dice,
            damage_type: self.damage_type,
            weapon_type: self.weapon_type,
            weapon_group: self.weapon_group,
            traits: self.traits,
            hands: self.hands,
            on_hit_effect: self.on_hit_effect,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WeaponType {
    Unarmed,
    Simple,
    Martial,
    Advanced,
}

#[derive(Debug, PartialEq)]
pub enum WeaponGroup {
    Unarmed,
    Sword,
    Knife,
}

#[derive(Debug, PartialEq)]
pub enum WeaponTrait {
    /// The multiple attack penalty you take with this weapon on the second
    /// attack on your turn is –4 instead of –5, and –8 instead of –10 on the
    /// third and subsequent attacks in the turn.
    Agile,

    /// You can use your Dexterity modifier instead of your Strength modifier
    /// on attack rolls using this melee weapon. You still calculate damage
    /// using Strength.
    Finesse,

    VersatilePiercing,
    VersatileSlashing,
    VersatileBludgeoning,

    Nonlethal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dagger() {
        let w = dagger();

        for _ in 0..100 {
            let damage = w.dice.sum();
            assert!(damage >= 1 && damage <= 4)
        }
    }

    #[test]
    fn test_shortsword() {
        let w = shortsword();

        for _ in 0..100 {
            let damage = w.dice.sum();
            assert!(damage >= 1 && damage <= 6)
        }
    }

    #[test]
    fn test_longsword() {
        let w = longsword();

        for _ in 0..100 {
            let damage = w.dice.sum();
            assert!(damage >= 1 && damage <= 8)
        }
    }
}
