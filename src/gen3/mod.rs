use crate::encounter::Encounter;
use crate::gen3::encounter_area_3::EncounterArea3;
use crate::slot::Slot;
use pkhex_rs::personal_info_g3::PersonalInfoG3;
use pkhex_rs::personal_table::PersonalTable;
use pkhex_rs::GameVersion;

pub mod encounter_area_3;

const EMERALD_RAW: &[u8] = include_bytes!("../resources/bin/gen3/emerald.bin");
const FIRE_RED_RAW: &[u8] = include_bytes!("../resources/bin/gen3/firered.bin");
const LEAF_GREEN_RAW: &[u8] = include_bytes!("../resources/bin/gen3/leafgreen.bin");
const RUBY_RAW: &[u8] = include_bytes!("../resources/bin/gen3/ruby.bin");
const SAPPHIRE_RAW: &[u8] = include_bytes!("../resources/bin/gen3/sapphire.bin");

fn get_areas(
    encounter: Encounter,
    game: GameVersion,
    info: &'static PersonalTable<PersonalInfoG3>,
) -> Vec<EncounterArea3> {
    let data = match game {
        GameVersion::FR => FIRE_RED_RAW,
        GameVersion::LG => LEAF_GREEN_RAW,
        GameVersion::E => EMERALD_RAW,
        GameVersion::R => RUBY_RAW,
        GameVersion::S => SAPPHIRE_RAW,
        _ => FIRE_RED_RAW,
    };

    let mut encounters = Vec::new();

    for index in (0..data.len()).step_by(121) {
        let entry = &data[index..];
        let location = entry[0];
        let grass = entry[1];
        let water = entry[2];
        let rock = entry[3];
        let fish = entry[4];

        match encounter {
            Encounter::Grass => {
                if grass != 0 {
                    let mut slots = Vec::with_capacity(12);
                    for i in 0..12 {
                        let level = entry[5 + (i * 3)];
                        let species = u16::from_le_bytes(
                            (entry[(6 + (i * 3))..(8 + (i * 3))]).try_into().unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: level,
                            max_level: level,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: grass,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::Surfing => {
                if water != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[41 + (i * 4)];
                        let max = entry[42 + (i * 4)];
                        let species = u16::from_le_bytes(
                            (entry[(43 + (i * 4))..(45 + (i * 4))]).try_into().unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: water,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::RockSmash => {
                if rock != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[61 + (i * 4)];
                        let max = entry[62 + (i * 4)];
                        let species = u16::from_le_bytes(
                            (entry[(63 + (i * 4))..(65 + (i * 4))]).try_into().unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: rock,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::OldRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(2);
                    for i in 0..2 {
                        let min = entry[81 + (i * 4)];
                        let max = entry[82 + (i * 4)];
                        let species = u16::from_le_bytes(
                            (entry[(83 + (i * 4))..(85 + (i * 4))]).try_into().unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: fish,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::GoodRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(3);
                    for i in 0..3 {
                        let min = entry[89 + (i * 4)];
                        let max = entry[90 + (i * 4)];
                        let species = u16::from_le_bytes(
                            (entry[(91 + (i * 4))..(93 + (i * 4))]).try_into().unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: fish,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::SuperRod => {
                if fish != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[101 + (i * 4)];
                        let max = entry[102 + (i * 4)];
                        let species = u16::from_le_bytes(
                            (entry[(103 + (i * 4))..(105 + (i * 4))])
                                .try_into()
                                .unwrap(),
                        );
                        let slot: Slot<PersonalInfoG3> = Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: &info.get_form_entry(species as usize, 0),
                        };
                        slots.push(slot);
                    }
                    encounters.push(EncounterArea3 {
                        location,
                        rate: fish,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            _ => {}
        }
    }

    encounters
}

pub fn get_encounters(encounter: Encounter, version: GameVersion) -> Vec<EncounterArea3> {
    let info: &PersonalTable<PersonalInfoG3> = match version {
        GameVersion::FR => &pkhex_rs::personal_table::FR,
        GameVersion::LG => &pkhex_rs::personal_table::LG,
        GameVersion::E => &pkhex_rs::personal_table::E,
        GameVersion::R | GameVersion::S => &pkhex_rs::personal_table::RS,
        _ => &pkhex_rs::personal_table::FR,
    };

    get_areas(encounter, version, info)
}
