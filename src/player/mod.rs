use crate::{
    ancestry::Ancestry, attribute::Attribute, attributes::Attributes, class::Class, dice::Dice,
    equipment::Equipment, items::weapons::Weapon, skills::Skills,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Player {
    level: i8,

    ancestry: Ancestry,
    class: Class,
    attributes: Attributes,
    skills: Skills,

    equipment: Equipment,

    cur_hp: i64,
}

pub struct PlayerBuilder {
    level: i8,

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

    pub fn level(mut self, level: i8) -> Self {
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
        let class_based =
            self.class.hp + self.get_final_attributes().get(Attribute::Constitution) as i64;
        let level_based = class_based * self.level as i64;

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

    fn get_active_weapon(&self) -> &Weapon {
        // TODO: Better logic here, if there are multiple weapons.
        if self.equipment.equipped_weapons.len() == 0 {
            return &self.equipment.natural_weapons[0];
        } else {
            return &self.equipment.equipped_weapons[0];
        }
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
}

#[cfg(test)]
mod tests {
    use crate::items::weapons;

    use super::*;

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
                    .build(),
            )
            .build()
            .unwrap()
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
    fn test_max_hp() {
        let p = construct_player();

        assert_eq!(p.get_max_hp(), 19);
    }
}
