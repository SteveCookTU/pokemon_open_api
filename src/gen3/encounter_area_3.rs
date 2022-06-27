use crate::encounter::Encounter;
use crate::encounter_area::EncounterArea;
use crate::slot::Slot;
use pkhex_rs::personal_info_g3::PersonalInfoG3;

pub struct EncounterArea3 {
    pub(crate) location: u8,
    pub(crate) rate: u8,
    pub(crate) encounter: Encounter,
    pub(crate) pokemon: Vec<Slot<PersonalInfoG3>>,
}

impl EncounterArea<PersonalInfoG3> for EncounterArea3 {
    fn get_encounter(&self) -> Encounter {
        self.encounter
    }

    fn get_location(&self) -> u8 {
        self.location
    }

    fn get_rate(&self) -> u8 {
        self.rate
    }

    fn get_pokemon(&self) -> &Vec<Slot<PersonalInfoG3>> {
        &self.pokemon
    }
}

impl EncounterArea3 {
    pub fn rse_safari_zone(&self) -> bool {
        match self.location {
            90 | 187 | 89 | 186 | 92 | 189 | 91 | 188 | 73 | 98 | 74 | 20 | 97 | 72 => true,
            _ => false,
        }
    }
}
