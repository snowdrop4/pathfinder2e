use crate::{damage_types::DamageType, dice::Dice, player::Player};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Weapon {
    name: String,
    price: u32, // in copper pieces
    dice: Dice,
    damage_type: Vec<DamageType>,
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
            price: 0,
            dice: Dice::D8,
            damage_type: Vec::new(),
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
    price: u32,
    dice: Dice,
    damage_type: Vec<DamageType>,
    weapon_type: WeaponType,
    weapon_group: WeaponGroup,
    traits: Vec<WeaponTrait>,
    hands: u8,
    on_hit_effect: Option<fn(&Weapon, &mut Player)>,
}

impl WeaponBuilder {
    pub fn price(mut self, price: u32) -> Self {
        self.price = price;
        self
    }

    pub fn dice(mut self, dice: Dice) -> Self {
        self.dice = dice;
        self
    }

    pub fn damage_type(mut self, damage_type: Vec<DamageType>) -> Self {
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

    fn construct_weapon() -> Weapon {
        Weapon::new("Dagger")
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
            .build()
    }

    #[test]
    fn test_damage() {
        let w = construct_weapon();

        for _ in 0..100 {
            let damage = w.dice.sum(1);
            assert!(damage >= 1 && damage <= 4)
        }
    }
}
