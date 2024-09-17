use crate::items::weapons::Weapon;

#[derive(Debug)]
pub struct Equipment {
    /// Unarmed equipment (i.e., fists for claws)
    pub unarmed: Vec<Weapon>,

    /// Equipment actively held in hand (i.e., a sword)
    pub hands: Vec<Weapon>,
}

impl Equipment {
    pub fn new() -> EquipmentBuilder {
        EquipmentBuilder {
            unarmed: Vec::new(),
            hands: Vec::new(),
        }
    }
}
pub struct EquipmentBuilder {
    pub unarmed: Vec<Weapon>,
    pub hands: Vec<Weapon>,
}

impl EquipmentBuilder {
    pub fn unarmed(mut self, unarmed: Vec<Weapon>) -> EquipmentBuilder {
        self.unarmed = unarmed;
        self
    }

    pub fn hands(mut self, hands: Vec<Weapon>) -> EquipmentBuilder {
        self.hands = hands;
        self
    }

    pub fn build(self) -> Equipment {
        Equipment {
            unarmed: self.unarmed,
            hands: self.hands,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::items::weapons;

    use super::Equipment;

    #[test]
    fn test_construct_equipment() {
        Equipment::new()
            .hands(vec![weapons::shortsword(), weapons::shortsword()])
            .build();
    }
}
