#[derive(Debug)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[allow(dead_code)]
impl Size {
    fn space(&self) -> u8 {
        match self {
            Size::Tiny => 2, // RAW: "less than 5 feet". Choosing to halve and round down.
            Size::Small | Size::Medium => 5,
            Size::Large => 10,
            Size::Huge => 15,
            Size::Gargantuan => 20, // RAW: "20 feet or more". Choosing 20.
        }
    }

    fn reach_tall(&self) -> u8 {
        match self {
            Size::Tiny => 0,
            Size::Small | Size::Medium => 5,
            Size::Large => 10,
            Size::Huge => 15,
            Size::Gargantuan => 20,
        }
    }

    fn reach_long(&self) -> u8 {
        match self {
            Size::Tiny => 0,
            Size::Small | Size::Medium | Size::Large => 5,
            Size::Huge => 10,
            Size::Gargantuan => 15,
        }
    }
}
