pub mod ancestry;
pub mod attribute;
pub mod attributes;
pub mod class;
pub mod currency;
pub mod damage_types;
pub mod dice;
pub mod equipment;
pub mod items;
pub mod player;
pub mod proficiency;
pub mod size;
pub mod skills;

use ancestry::Ancestry;
use attribute::Attribute;
use class::Class;
use equipment::Equipment;
use items::weapons;
use player::Player;

fn main() {}

#[test]
fn test_combat() {
    let p1 = Player::new()
        .ancestry(Ancestry::new_human((
            Attribute::Dexterity,
            Attribute::Constitution,
        )))
        .class(Class::new_fighter())
        .equipment(Equipment::new().hands(vec![weapons::dagger()]).build())
        .build()
        .unwrap();

    let p2 = Player::new()
        .ancestry(Ancestry::new_human((
            Attribute::Dexterity,
            Attribute::Constitution,
        )))
        .class(Class::new_fighter())
        .equipment(Equipment::new().hands(vec![weapons::dagger()]).build())
        .build()
        .unwrap();
}
