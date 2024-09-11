use crate::items::weapons::Weapon;

#[derive(Debug)]
pub struct Equipment {
    pub hands: Vec<Weapon>,
}

impl Equipment {
    pub fn new() -> EquipmentBuilder {
        EquipmentBuilder { hands: Vec::new() }
    }
}
pub struct EquipmentBuilder {
    pub hands: Vec<Weapon>,
}

impl EquipmentBuilder {
    pub fn hands(mut self, hands: Vec<Weapon>) -> EquipmentBuilder {
        self.hands = hands;
        self
    }

    pub fn build(self) -> Equipment {
        Equipment { hands: self.hands }
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
