use rand::Rng;

#[derive(Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

#[allow(dead_code)]
impl Dice {
    pub fn roll(&self, num_dice: u8) -> Vec<i64> {
        let mut rng = rand::thread_rng();

        let die_size = match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
        };

        (0..num_dice).map(|_| rng.gen_range(1..=die_size)).collect()
    }

    pub fn sum(&self, num_dice: u8) -> i64 {
        self.roll(num_dice).iter().sum()
    }
}

#[derive(Debug)]
pub struct DiceAmount {
    pub n: u8,
    pub d: Dice,
}

impl DiceAmount {
    pub fn roll(&self) -> Vec<i64> {
        self.d.roll(self.n)
    }

    pub fn sum(&self) -> i64 {
        self.d.sum(self.n)
    }
}
