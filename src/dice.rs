use rand::Rng;

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
    pub fn roll(&self, num_dice: u32) -> Vec<u32> {
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

    pub fn sum(&self, num_dice: u32) -> u32 {
        let rolls = self.roll(num_dice);
        let sum: u32 = rolls.iter().sum();
        sum
    }
}
