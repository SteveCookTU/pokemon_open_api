use pkhex_rs::PersonalInfo;

pub struct Slot<T: PersonalInfo + 'static> {
    pub(crate) min_level: u8,
    pub(crate) max_level: u8,
    pub(crate) species: u16,
    pub(crate) info: &'static T,
}

impl<T: PersonalInfo> Slot<T> {
    pub fn get_min_level(&self) -> u8 {
        self.min_level
    }

    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    pub fn get_species(&self) -> u16 {
        self.species
    }

    pub fn get_info(&self) -> &'static T {
        self.info
    }

    pub fn set_species(&mut self, species: u16, info: &'static T) {
        self.species = species;
        self.info = info;
    }
}
