use crate::encounter::Encounter;
use crate::encounter_area::EncounterArea;
use crate::slot::Slot;
use pkhex_rs::personal_info_g4::PersonalInfoG4;

pub struct EncounterArea4 {
    pub(crate) location: u8,
    pub(crate) rate: u8,
    pub(crate) encounter: Encounter,
    pub(crate) pokemon: Vec<Slot<PersonalInfoG4>>,
}

impl EncounterArea<PersonalInfoG4> for EncounterArea4 {
    fn get_encounter(&self) -> Encounter {
        self.encounter
    }

    fn get_location(&self) -> u8 {
        self.location
    }

    fn get_rate(&self) -> u8 {
        self.rate
    }

    fn get_pokemon(&self) -> &Vec<Slot<PersonalInfoG4>> {
        &self.pokemon
    }
}
