use core::panic;
use std::cmp::max;

use crate::ancestry::Ancestry;
use crate::attribute::Attribute;
use crate::attributes::Attributes;
use crate::class::Class;
use crate::dice::Dice;
use crate::equipment::Equipment;
use crate::items::armors::Armor;
use crate::items::weapons::Weapon;
use crate::skills::Skills;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    level: i64,

    ancestry: Ancestry,
    class: Class,
    attributes: Attributes,
    skills: Skills,

    equipment: Equipment,

    cur_hp: i64,
}

pub struct PlayerBuilder {
    level: i64,

    ancestry: Option<Ancestry>,
    class: Option<Class>,
    attributes: Option<Attributes>,
    skills: Option<Skills>,

    equipment: Option<Equipment>,
}

impl PlayerBuilder {
    pub fn ancestry(mut self, ancestry: Ancestry) -> Self {
        self.ancestry = Some(ancestry);
        self
    }

    pub fn class(mut self, class: Class) -> Self {
        self.class = Some(class);
        self
    }

    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = Some(attributes);
        self
    }

    pub fn skills(mut self, skills: Skills) -> Self {
        self.skills = Some(skills);
        self
    }

    pub fn equipment(mut self, equipment: Equipment) -> Self {
        self.equipment = Some(equipment);
        self
    }

    pub fn level(mut self, level: i64) -> Self {
        self.level = level;
        self
    }

    pub fn build(self) -> Result<Player, &'static str> {
        let ancestry = self.ancestry.ok_or("Ancestry is required")?;
        let class = self.class.ok_or("Class is required")?;
        let attributes = self.attributes.unwrap_or_else(Attributes::new);
        let skills = self.skills.unwrap_or_else(Skills::new);
        let equipment = self.equipment.unwrap_or_else(|| Equipment::new().build());

        let mut player = Player {
            level: self.level,
            ancestry,
            class,
            attributes,
            skills,
            equipment,
            cur_hp: 0,
        };

        player.cur_hp = player.get_max_hp();

        Ok(player)
    }
}

impl Player {
    pub fn new() -> PlayerBuilder {
        PlayerBuilder {
            level: 1,
            ancestry: None,
            class: None,
            attributes: None,
            skills: None,
            equipment: None,
        }
    }

    pub fn get_max_hp(&self) -> i64 {
        let class_based = self.class.hp + self.get_final_attributes().get(Attribute::Constitution);
        let level_based = class_based * self.level;

        self.ancestry.hp + level_based
    }

    pub fn get_final_attributes(&self) -> Attributes {
        self.attributes.get_final(
            &[
                self.ancestry.attribute_boosts.0,
                self.ancestry.attribute_boosts.1,
            ],
            &[],
        )
    }

    /// While wearing a suit of armor, you take the penalty listed in this
    /// entry to your Speed, as well as to any other movement types you have,
    /// such as a climb Speed or swim Speed, to a minimum Speed of 5 feet.
    fn get_stride_speed(&self) -> i64 {
        let speed = if self.armor_requirements_met() {
            self.ancestry.stride_speed
        } else {
            self.ancestry.stride_speed - self.get_active_armor().speed_penalty
        };

        max(5, speed)
    }

    fn get_active_weapon(&self) -> &Weapon {
        // TODO: Better logic here, if there are multiple weapons.
        if self.equipment.equipped_weapons.len() != 0 {
            return &self.equipment.equipped_weapons[0];
        }

        if self.equipment.natural_weapons.len() != 0 {
            return &self.equipment.natural_weapons[0];
        }

        panic!("no equipped weapons")
    }

    fn get_active_armor(&self) -> &Armor {
        if let Some(ref armor) = self.equipment.equipped_armor {
            return armor;
        }

        if let Some(ref armor) = self.equipment.natural_armor {
            return &armor;
        }

        panic!("no equipped armor")
    }

    pub fn weapon_attack(&self, attack_index: i64) -> i64 {
        let roll = Dice::D20.sum(1);

        let weapon = self.get_active_weapon();

        // TODO: weapon_attack_mod
        let attribute_attack_mod = weapon.get_attribute_attack_mod(&self.get_final_attributes());

        let map = weapon.get_multiple_attack_penalty(attack_index);

        roll + attribute_attack_mod + map
    }

    pub fn weapon_damage(&self) -> i64 {
        let roll = Dice::D20.sum(1);

        let weapon = self.get_active_weapon();

        // TODO: weapon_damage_mod
        let attribute_damage_mod = weapon.get_attribute_damage_mod(&self.get_final_attributes());

        return roll + attribute_damage_mod;
    }

    fn armor_requirements_met(&self) -> bool {
        let armor = self.get_active_armor();

        return armor.strength_required.unwrap_or(i64::MIN)
            > self.get_final_attributes().get(Attribute::Strength);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::items::armors::skin;
    use crate::items::weapons;

    fn construct_player() -> Player {
        Player::new()
            .ancestry(Ancestry::new_human((
                Attribute::Dexterity,
                Attribute::Constitution,
            )))
            .class(Class::new_fighter())
            .equipment(
                Equipment::new()
                    .equipped_weapons(vec![weapons::dagger()])
                    .equipped_armor(Some(skin()))
                    .build(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn test_max_hp() {
        let p = construct_player();

        assert_eq!(p.get_max_hp(), 19);
    }

    #[test]
    fn test_attribute_boosts() {
        let p = construct_player();

        let a = p.get_final_attributes();

        assert_eq!(a.get(Attribute::Strength), 0);
        assert_eq!(a.get(Attribute::Dexterity), 1);
        assert_eq!(a.get(Attribute::Constitution), 1);
        assert_eq!(a.get(Attribute::Intelligence), 0);
        assert_eq!(a.get(Attribute::Wisdom), 0);
        assert_eq!(a.get(Attribute::Charisma), 0);
    }

    #[test]
    fn test_stride_speed() {
        let p = construct_player();

        assert_eq!(p.get_stride_speed(), 25);
    }
}
