pub mod definitions;
pub use definitions::*;

use crate::currency::{cp, Currency};

#[derive(Debug)]
pub enum ArmorType {
    Unarmored,
    Light,
    Medium,
    Heavy,
}

#[derive(Debug)]
pub enum ArmorTrait {
    Noisy,
    Flexible,
    Comfortable,
    Bulwark,
    // Add more traits as needed
}

#[derive(Debug)]
pub struct Armor {
    pub name: String,
    pub price: Currency,
    pub armor_type: ArmorType,
    pub ac_bonus: i64,
    pub dex_cap: Option<i64>,

    /// This entry indicates the Strength modifier at which you are strong
    /// enough to overcome some of the armor’s penalties. If your Strength
    /// modifier is equal to or greater than this value, you no longer take
    /// the armor’s check penalty, and you decrease the Speed penalty by 5
    /// feet (to no penalty if the penalty was –5 feet, or to a –5-foot penalty
    /// if the penalty was –10 feet).
    pub strength_required: Option<i64>,

    /// While wearing your armor, you take this penalty to Strength- and
    /// Dexterity-based skill checks, except for those that have the attack trait.
    /// If you meet the armor’s Strength threshold, you don’t take this penalty.
    pub check_penalty: i64,

    /// While wearing a suit of armor, you take the penalty listed in this entry
    /// to your Speed, as well as to any other movement types you have, such as
    /// a climb Speed or swim Speed, to a minimum Speed of 5 feet. If you meet
    /// the armor’s Strength threshold (see below), you reduce the penalty by 5 feet.
    pub speed_penalty: i64,

    pub traits: Vec<ArmorTrait>,
}

impl Armor {
    pub fn new(name: &str) -> ArmorBuilder {
        ArmorBuilder {
            name: name.to_string(),
            price: cp(1),
            armor_type: ArmorType::Light,
            ac_bonus: 1,
            dex_cap: None,
            strength_required: None,
            check_penalty: 0,
            speed_penalty: 0,
            traits: Vec::new(),
        }
    }
}

pub struct ArmorBuilder {
    name: String,
    price: Currency,
    armor_type: ArmorType,
    ac_bonus: i64,
    dex_cap: Option<i64>,
    strength_required: Option<i64>,
    check_penalty: i64,
    speed_penalty: i64,
    traits: Vec<ArmorTrait>,
}

impl ArmorBuilder {
    pub fn price(mut self, price: Currency) -> Self {
        self.price = price;
        self
    }

    pub fn armor_type(mut self, armor_type: ArmorType) -> Self {
        self.armor_type = armor_type;
        self
    }

    pub fn ac_bonus(mut self, ac_bonus: i64) -> Self {
        self.ac_bonus = ac_bonus;
        self
    }

    pub fn dex_cap(mut self, dex_cap: Option<i64>) -> Self {
        self.dex_cap = dex_cap;
        self
    }

    pub fn strength_required(mut self, strength_required: Option<i64>) -> Self {
        self.strength_required = strength_required;
        self
    }

    pub fn check_penalty(mut self, check_penalty: i64) -> Self {
        self.check_penalty = check_penalty;
        self
    }

    pub fn speed_penalty(mut self, speed_penalty: i64) -> Self {
        self.speed_penalty = speed_penalty;
        self
    }

    pub fn traits(mut self, traits: Vec<ArmorTrait>) -> Self {
        self.traits = traits;
        self
    }

    pub fn build(self) -> Armor {
        Armor {
            name: self.name,
            price: self.price,
            armor_type: self.armor_type,
            ac_bonus: self.ac_bonus,
            dex_cap: self.dex_cap,
            strength_required: self.strength_required,
            check_penalty: self.check_penalty,
            speed_penalty: self.speed_penalty,
            traits: self.traits,
        }
    }
}
