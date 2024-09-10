use rand::Rng;

use crate::damage_types::DamageType;

#[derive(Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
}

#[allow(dead_code)]
impl Dice {
    pub fn roll(&self, num_dice: u8) -> Vec<u32> {
        let mut rng = rand::thread_rng();

        let die_size = match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
        };

        (0..num_dice).map(|_| rng.gen_range(1..=die_size)).collect()
    }

    pub fn sum(&self, num_dice: u8) -> u32 {
        let rolls = self.roll(num_dice);
        let sum: u32 = rolls.iter().sum();
        sum
    }
}

#[derive(Debug)]
pub struct DiceAmount {
    pub n: u8,
    pub d: Dice,
}

impl DiceAmount {
    pub fn roll(&self) -> Vec<u32> {
        self.d.roll(self.n)
    }

    pub fn sum(&self) -> u32 {
        self.d.sum(self.n)
    }
}

#[derive(Debug)]
pub struct DamageAmount {
    pub n: u8,
    pub d: Dice,
    pub t: DamageType,
}

impl DamageAmount {
    pub fn roll(&self) -> Vec<u32> {
        self.d.roll(self.n)
    }

    pub fn sum(&self) -> u32 {
        self.d.sum(self.n)
    }
}
