use crate::attribute::Attribute;

#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    values: [i64; 6],
}

impl Attributes {
    pub fn new() -> Self {
        Attributes { values: [0; 6] }
    }

    pub fn get_final(self, boosts: &[Attribute], hinders: &[Attribute]) -> Attributes {
        let mut attributes = Attributes::new();

        for attr in boosts {
            attributes.values[*attr as usize] += 1;
        }
        for attr in hinders {
            attributes.values[*attr as usize] -= 1;
        }
        attributes
    }

    pub fn get(&self, attr: Attribute) -> i64 {
        self.values[attr as usize]
    }
}
