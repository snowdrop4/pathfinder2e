use crate::proficiency::Proficiency;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Skills {
    acrobatics: Proficiency,
    arcana: Proficiency,
    athletics: Proficiency,
    crafting: Proficiency,
    deception: Proficiency,
    diplomacy: Proficiency,
    intimidation: Proficiency,
    lore: Proficiency,
    medicine: Proficiency,
    nature: Proficiency,
    occultism: Proficiency,
    performance: Proficiency,
    religion: Proficiency,
    society: Proficiency,
    stealth: Proficiency,
    survival: Proficiency,
    thievery: Proficiency,
}

impl Skills {
    pub fn new() -> Skills {
        return Skills {
            acrobatics: Proficiency::UNTRAINED,
            arcana: Proficiency::UNTRAINED,
            athletics: Proficiency::UNTRAINED,
            crafting: Proficiency::UNTRAINED,
            deception: Proficiency::UNTRAINED,
            diplomacy: Proficiency::UNTRAINED,
            intimidation: Proficiency::UNTRAINED,
            lore: Proficiency::UNTRAINED,
            medicine: Proficiency::UNTRAINED,
            nature: Proficiency::UNTRAINED,
            occultism: Proficiency::UNTRAINED,
            performance: Proficiency::UNTRAINED,
            religion: Proficiency::UNTRAINED,
            society: Proficiency::UNTRAINED,
            stealth: Proficiency::UNTRAINED,
            survival: Proficiency::UNTRAINED,
            thievery: Proficiency::UNTRAINED,
        };
    }
}
