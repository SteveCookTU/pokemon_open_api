use crate::encounter::Encounter;
use crate::slot::Slot;
use pkhex_rs::PersonalInfo;
use std::ops::Range;

pub trait EncounterArea<T: PersonalInfo + 'static> {
    fn get_encounter(&self) -> Encounter;

    fn get_location(&self) -> u8;

    fn get_rate(&self) -> u8;

    fn get_pokemon(&self) -> &Vec<Slot<T>>;

    fn get_unique_species(&self) -> Vec<u16> {
        let mut species = Vec::new();
        for p in self.get_pokemon() {
            if !species.contains(&p.get_species()) {
                species.push(p.get_species());
            }
        }

        species
    }

    fn get_slots(&self, num: u16) -> Vec<bool> {
        let mut flags = Vec::with_capacity(self.get_pokemon().len());
        for p in self.get_pokemon() {
            flags.push(p.get_species() == num);
        }

        flags
    }

    fn get_level_range(&self, species: u16) -> Range<u8> {
        let mut range = 100..0;
        println!("{} {}", range.start, range.end);
        for p in self.get_pokemon() {
            if p.get_species() == species {
                range.start = (range.start as u8).min(p.get_min_level());
                range.end = (range.end as u8).max(p.get_max_level());
            }
        }

        range
    }

    fn get_species_name(&self) -> Vec<String> {
        let mut names = Vec::new();

        for p in self.get_pokemon() {
            names.push(pkhex_rs::game_strings::SPECIES_EN[p.get_species() as usize].to_string());
        }

        names
    }
}
