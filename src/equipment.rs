use crate::items::armors::Armor;
use crate::items::weapons::Weapon;

#[derive(Debug)]
pub struct Equipment {
    /// Weapons inherent to the creature (i.e., fists or claws)
    pub natural_weapons: Vec<Weapon>,

    /// Weapons actively held in hand (i.e., a sword)
    pub equipped_weapons: Vec<Weapon>,

    /// Armor inherent to the creature (i.e., scales)
    pub natural_armor: Option<Armor>,

    /// Armor actively worn on body (i.e., a breastplate)
    pub equipped_armor: Option<Armor>,
}

impl Equipment {
    pub fn new() -> EquipmentBuilder {
        EquipmentBuilder {
            natural_weapons: Vec::new(),
            equipped_weapons: Vec::new(),
            natural_armor: None,
            equipped_armor: None,
        }
    }
}
pub struct EquipmentBuilder {
    pub natural_weapons: Vec<Weapon>,
    pub equipped_weapons: Vec<Weapon>,
    pub natural_armor: Option<Armor>,
    pub equipped_armor: Option<Armor>,
}

impl EquipmentBuilder {
    pub fn natural_weapons(mut self, unarmed_weapons: Vec<Weapon>) -> EquipmentBuilder {
        self.natural_weapons = unarmed_weapons;
        self
    }

    pub fn equipped_weapons(mut self, equipped_weapons: Vec<Weapon>) -> EquipmentBuilder {
        self.equipped_weapons = equipped_weapons;
        self
    }

    pub fn natural_armor(mut self, natural_armor: Option<Armor>) -> EquipmentBuilder {
        self.natural_armor = natural_armor;
        self
    }

    pub fn equipped_armor(mut self, equipped_armor: Option<Armor>) -> EquipmentBuilder {
        self.equipped_armor = equipped_armor;
        self
    }

    pub fn build(self) -> Equipment {
        Equipment {
            natural_weapons: self.natural_weapons,
            equipped_weapons: self.equipped_weapons,
            natural_armor: self.natural_armor,
            equipped_armor: self.equipped_armor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Equipment;
    use crate::items::{armors, weapons};

    #[test]
    fn test_construct_equipment() {
        Equipment::new()
            .natural_weapons(vec![weapons::fists()])
            .equipped_weapons(vec![weapons::shortsword(), weapons::shortsword()])
            .natural_armor(Some(armors::skin()))
            .equipped_armor(None)
            .build();
    }
}
