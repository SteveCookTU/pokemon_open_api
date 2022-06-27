use crate::encounter::Encounter;
use crate::gen4::encounter_area_4::EncounterArea4;
use crate::slot::Slot;
use pkhex_rs::personal_info_g4::PersonalInfoG4;
use pkhex_rs::personal_table::PersonalTable;
use pkhex_rs::{personal_table, GameVersion};

pub mod encounter_area_4;

const HG_HEADBUTT_RAW: &[u8] = include_bytes!("../resources/bin/gen4/hg_headbutt.bin");
const SS_HEADBUTT_RAW: &[u8] = include_bytes!("../resources/bin/gen4/ss_headbutt.bin");
const HG_SS_BUG_RAW: &[u8] = include_bytes!("../resources/bin/gen4/hgss_bug.bin");
const HEART_GOLD_RAW: &[u8] = include_bytes!("../resources/bin/gen4/heartgold.bin");
const SOUL_SILVER_RAW: &[u8] = include_bytes!("../resources/bin/gen4/soulsilver.bin");
const DIAMOND_RAW: &[u8] = include_bytes!("../resources/bin/gen4/diamond.bin");
const PEARL_RAW: &[u8] = include_bytes!("../resources/bin/gen4/pearl.bin");
const PLATINUM_RAW: &[u8] = include_bytes!("../resources/bin/gen4/platinum.bin");

pub struct Profile4 {
    pub version: GameVersion,
    pub has_national_dex: bool,
    pub radio: usize,
    pub swarm: bool,
    pub radar: bool,
    pub dual: GameVersion,
}

fn modify_radio(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    radio: usize,
) {
    let (species_1, species_2) = if radio == 1 {
        (
            u16::from_le_bytes((&data[91..93]).try_into().unwrap()),
            u16::from_le_bytes((&data[93..95]).try_into().unwrap()),
        )
    } else if radio == 2 {
        (
            u16::from_le_bytes((&data[95..97]).try_into().unwrap()),
            u16::from_le_bytes((&data[97..99]).try_into().unwrap()),
        )
    } else {
        return;
    };

    mons[2].set_species(species_1, table.get_form_entry(species_1 as usize, 0));
    mons[3].set_species(species_1, table.get_form_entry(species_1 as usize, 0));
    mons[4].set_species(species_2, table.get_form_entry(species_2 as usize, 0));
    mons[5].set_species(species_2, table.get_form_entry(species_2 as usize, 0));
}

fn modify_swarm_hgss(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    encounter: Encounter,
    swarm: bool,
) {
    if swarm {
        match encounter {
            Encounter::Grass => {
                let species = u16::from_le_bytes((&data[187..189]).try_into().unwrap());
                mons[0].set_species(species, table.get_form_entry(species as usize, 0));
                mons[1].set_species(species, table.get_form_entry(species as usize, 0));
            }
            Encounter::Surfing => {
                let species = u16::from_le_bytes((&data[189..191]).try_into().unwrap());
                mons[0].set_species(species, table.get_form_entry(species as usize, 0));
            }
            Encounter::OldRod => {
                let species = u16::from_le_bytes((&data[193..195]).try_into().unwrap());
                mons[2].set_species(species, table.get_form_entry(species as usize, 0));
            }
            Encounter::GoodRod => {
                let species = u16::from_le_bytes((&data[193..195]).try_into().unwrap());
                mons[0].set_species(species, table.get_form_entry(species as usize, 0));
                mons[2].set_species(species, table.get_form_entry(species as usize, 0));
                mons[3].set_species(species, table.get_form_entry(species as usize, 0));
            }
            Encounter::SuperRod => {
                let species = u16::from_le_bytes((&data[193..195]).try_into().unwrap());
                mons[0].set_species(species, table.get_form_entry(species as usize, 0));
                mons[1].set_species(species, table.get_form_entry(species as usize, 0));
                mons[2].set_species(species, table.get_form_entry(species as usize, 0));
                mons[3].set_species(species, table.get_form_entry(species as usize, 0));
                mons[4].set_species(species, table.get_form_entry(species as usize, 0));
            }
            _ => {}
        };
    }
}

fn modify_swarm_dppt(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    swarm: bool,
) {
    if swarm {
        let species_1 = u16::from_le_bytes((&data[38..40]).try_into().unwrap());
        let species_2 = u16::from_le_bytes((&data[40..42]).try_into().unwrap());
        mons[0].set_species(species_1, table.get_form_entry(species_1 as usize, 0));
        mons[1].set_species(species_2, table.get_form_entry(species_2 as usize, 0));
    }
}

fn modify_time(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    time: usize,
) {
    let (species_1, species_2) = if time == 1 {
        (
            u16::from_le_bytes((&data[42..44]).try_into().unwrap()),
            u16::from_le_bytes((&data[44..46]).try_into().unwrap()),
        )
    } else if time == 2 {
        (
            u16::from_le_bytes((&data[46..48]).try_into().unwrap()),
            u16::from_le_bytes((&data[48..50]).try_into().unwrap()),
        )
    } else {
        return;
    };

    mons[2].set_species(species_1, table.get_form_entry(species_1 as usize, 0));
    mons[3].set_species(species_2, table.get_form_entry(species_2 as usize, 0));
}

fn modify_radar(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    radar: bool,
) {
    if radar {
        let species = [
            u16::from_le_bytes((&data[50..52]).try_into().unwrap()),
            u16::from_le_bytes((&data[52..54]).try_into().unwrap()),
            u16::from_le_bytes((&data[54..56]).try_into().unwrap()),
            u16::from_le_bytes((&data[56..58]).try_into().unwrap()),
        ];
        mons[4].set_species(species[0], table.get_form_entry(species[0] as usize, 0));
        mons[5].set_species(species[1], table.get_form_entry(species[1] as usize, 0));
        mons[10].set_species(species[2], table.get_form_entry(species[2] as usize, 0));
        mons[11].set_species(species[3], table.get_form_entry(species[3] as usize, 0));
    }
}

fn modify_dual(
    mons: &mut Vec<Slot<PersonalInfoG4>>,
    data: &[u8],
    table: &'static PersonalTable<PersonalInfoG4>,
    dual: GameVersion,
) {
    let (species_1, species_2) = if dual == GameVersion::R {
        (
            u16::from_le_bytes((&data[59..61]).try_into().unwrap()),
            u16::from_le_bytes((&data[61..63]).try_into().unwrap()),
        )
    } else if dual == GameVersion::S {
        (
            u16::from_le_bytes((&data[63..65]).try_into().unwrap()),
            u16::from_le_bytes((&data[65..67]).try_into().unwrap()),
        )
    } else if dual == GameVersion::E {
        (
            u16::from_le_bytes((&data[67..69]).try_into().unwrap()),
            u16::from_le_bytes((&data[69..71]).try_into().unwrap()),
        )
    } else if dual == GameVersion::FR {
        (
            u16::from_le_bytes((&data[71..73]).try_into().unwrap()),
            u16::from_le_bytes((&data[73..75]).try_into().unwrap()),
        )
    } else if dual == GameVersion::LG {
        (
            u16::from_le_bytes((&data[75..77]).try_into().unwrap()),
            u16::from_le_bytes((&data[77..79]).try_into().unwrap()),
        )
    } else {
        return;
    };

    mons[8].set_species(species_1, table.get_form_entry(species_1 as usize, 0));
    mons[9].set_species(species_2, table.get_form_entry(species_2 as usize, 0));
}

fn get_hgss(
    version: GameVersion,
    encounter: Encounter,
    table: &'static PersonalTable<PersonalInfoG4>,
    profile: &Profile4,
    modifier: usize,
) -> Vec<EncounterArea4> {
    let mut encounters = Vec::new();

    if encounter == Encounter::Headbutt {
        let data = match version {
            GameVersion::HG => HG_HEADBUTT_RAW,
            _ => SS_HEADBUTT_RAW,
        };

        let mut offset = 0;
        while offset < data.len() {
            let entry = &data[offset..];
            let location = entry[0];
            let special_trees_flag = entry[1];
            let trees_type = if special_trees_flag == 0 && modifier == 2 {
                0
            } else {
                modifier
            };

            let mut slots = Vec::with_capacity(6);
            for i in 0..6 {
                let index = (24 * trees_type) + 2 + (i * 4);
                let species = u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                let min = entry[(24 * trees_type) + 4 + (i * 4)];
                let max = entry[(24 * trees_type) + 5 + (i * 4)];
                slots.push(Slot {
                    min_level: min,
                    max_level: max,
                    species,
                    info: table.get_form_entry(species as usize, 0),
                });
            }

            encounters.push(EncounterArea4 {
                location,
                rate: 0,
                encounter: Encounter::Headbutt,
                pokemon: slots,
            });

            offset += if special_trees_flag == 0 { 50 } else { 74 };
        }
    } else if encounter == Encounter::BugCatchingContest {
        let size = if profile.has_national_dex {
            HG_SS_BUG_RAW.len()
        } else {
            41
        };
        for offset in (0..size)
            .skip(if profile.has_national_dex { 40 } else { 0 })
            .step_by(41)
        {
            let entry = &HG_SS_BUG_RAW[offset..];
            let location = entry[0];

            let mut slots = Vec::with_capacity(10);
            for i in 0..10 {
                let index = 1 + (i * 4);
                let species = u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                let min = entry[3 + (i * 4)];
                let max = entry[4 + (i * 4)];
                slots.push(Slot {
                    min_level: min,
                    max_level: max,
                    species,
                    info: table.get_form_entry(species as usize, 0),
                });
            }

            encounters.push(EncounterArea4 {
                location,
                rate: 0,
                encounter: Encounter::BugCatchingContest,
                pokemon: slots,
            });
        }
    } else {
        let data = match version {
            GameVersion::HG => HEART_GOLD_RAW,
            _ => SOUL_SILVER_RAW,
        };

        for offset in (0..data.len()).step_by(195) {
            let entry = &data[offset..];
            let location = entry[0];
            let grass = entry[1];
            let surf = entry[2];
            let rock = entry[3];
            let old = entry[4];
            let good = entry[5];
            let srod = entry[6];

            match encounter {
                Encounter::Grass => {
                    if grass != 0 {
                        let mut slots = Vec::with_capacity(12);
                        for i in 0..12 {
                            let level = entry[7 + i];
                            let index = 19 + (i * 2) + (modifier * 24);
                            let species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );
                            slots.push(Slot {
                                min_level: level,
                                max_level: level,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }

                        modify_radio(&mut slots, entry, table, profile.radio);
                        modify_swarm_hgss(&mut slots, entry, table, encounter, profile.swarm);
                        encounters.push(EncounterArea4 {
                            location,
                            rate: grass,
                            encounter,
                            pokemon: vec![],
                        })
                    }
                }
                Encounter::Surfing => {
                    if surf != 0 {
                        let mut slots = Vec::with_capacity(5);
                        for i in 0..5 {
                            let min = entry[99 + (i * 4)];
                            let max = entry[100 + (i * 4)];
                            let index = 101 + (i * 4);
                            let species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );
                            slots.push(Slot {
                                min_level: min,
                                max_level: max,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }
                        modify_swarm_hgss(&mut slots, entry, table, encounter, profile.swarm);
                        encounters.push(EncounterArea4 {
                            location,
                            rate: surf,
                            encounter,
                            pokemon: slots,
                        });
                    }
                }
                Encounter::RockSmash => {
                    if rock != 0 {
                        let mut slots = Vec::with_capacity(2);
                        for i in 0..2 {
                            let min = entry[119 + (i * 4)];
                            let max = entry[120 + (i * 4)];
                            let index = 121 + (i * 4);
                            let species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );
                            slots.push(Slot {
                                min_level: min,
                                max_level: max,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }
                        encounters.push(EncounterArea4 {
                            location,
                            rate: rock,
                            encounter,
                            pokemon: slots,
                        });
                    }
                }
                Encounter::OldRod => {
                    if old != 0 {
                        let mut slots = Vec::with_capacity(5);
                        for i in 0..5 {
                            let min = entry[127 + (i * 4)];
                            let max = entry[128 + (i * 4)];
                            let index = 129 + (i * 4);
                            let species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );
                            slots.push(Slot {
                                min_level: min,
                                max_level: max,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }
                        modify_swarm_hgss(&mut slots, entry, table, encounter, profile.swarm);
                        encounters.push(EncounterArea4 {
                            location,
                            rate: old,
                            encounter,
                            pokemon: slots,
                        });
                    }
                }
                Encounter::GoodRod => {
                    if good != 0 {
                        let mut slots = Vec::with_capacity(5);
                        for i in 0..5 {
                            let min = entry[147 + (i * 4)];
                            let max = entry[148 + (i * 4)];
                            let index = 149 + (i * 4);
                            let mut species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );

                            if (modifier == 0 || modifier == 1) && i == 3 {
                                species =
                                    u16::from_le_bytes((&entry[191..193]).try_into().unwrap());
                            }

                            slots.push(Slot {
                                min_level: min,
                                max_level: max,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }
                        modify_swarm_hgss(&mut slots, entry, table, encounter, profile.swarm);
                        encounters.push(EncounterArea4 {
                            location,
                            rate: good,
                            encounter,
                            pokemon: slots,
                        });
                    }
                }
                Encounter::SuperRod => {
                    if srod != 0 {
                        let mut slots = Vec::with_capacity(5);
                        for i in 0..5 {
                            let min = entry[167 + (i * 4)];
                            let max = entry[168 + (i * 4)];
                            let index = 169 + (i * 4);
                            let mut species = u16::from_le_bytes(
                                (&entry[index..(index + 2)]).try_into().unwrap(),
                            );

                            if (modifier == 0 || modifier == 1) && i == 1 {
                                species =
                                    u16::from_le_bytes((&entry[191..193]).try_into().unwrap());
                            }

                            slots.push(Slot {
                                min_level: min,
                                max_level: max,
                                species,
                                info: table.get_form_entry(species as usize, 0),
                            });
                        }
                        modify_swarm_hgss(&mut slots, entry, table, encounter, profile.swarm);
                        encounters.push(EncounterArea4 {
                            location,
                            rate: srod,
                            encounter,
                            pokemon: slots,
                        });
                    }
                }
                _ => {}
            }
        }
    }
    encounters
}

fn get_dppt(
    version: GameVersion,
    encounter: Encounter,
    profile: &Profile4,
    table: &'static PersonalTable<PersonalInfoG4>,
    time: usize,
) -> Vec<EncounterArea4> {
    let data = match version {
        GameVersion::D => DIAMOND_RAW,
        GameVersion::P => PEARL_RAW,
        _ => PLATINUM_RAW,
    };

    let mut encounters = Vec::new();

    for offset in (0..data.len()).step_by(163) {
        let entry = &data[offset..];
        let location = entry[0];
        let grass = entry[1];
        let surf = entry[79];
        let old = entry[100];
        let good = entry[121];
        let srod = entry[142];

        match encounter {
            Encounter::Grass => {
                if grass != 0 {
                    let mut slots = Vec::with_capacity(12);
                    for i in 0..12 {
                        let level = entry[2 + (i * 3)];
                        let index = 3 + (i * 3);
                        let species =
                            u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                        slots.push(Slot {
                            min_level: level,
                            max_level: level,
                            species,
                            info: table.get_form_entry(species as usize, 0),
                        });
                    }
                    modify_swarm_dppt(&mut slots, entry, table, profile.swarm);
                    modify_time(&mut slots, entry, table, time);
                    modify_radar(&mut slots, entry, table, profile.radar);
                    modify_dual(&mut slots, entry, table, profile.dual);
                    encounters.push(EncounterArea4 {
                        location,
                        rate: grass,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::Surfing => {
                if surf != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[80 + (i * 4)];
                        let max = entry[81 + (i * 4)];
                        let index = 82 + (i * 4);
                        let species =
                            u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                        slots.push(Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: table.get_form_entry(species as usize, 0),
                        });
                    }
                    encounters.push(EncounterArea4 {
                        location,
                        rate: surf,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::OldRod => {
                if old != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[101 + (i * 4)];
                        let max = entry[102 + (i * 4)];
                        let index = 103 + (i * 4);
                        let species =
                            u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                        slots.push(Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: table.get_form_entry(species as usize, 0),
                        });
                    }
                    encounters.push(EncounterArea4 {
                        location,
                        rate: old,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::GoodRod => {
                if good != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[122 + (i * 4)];
                        let max = entry[123 + (i * 4)];
                        let index = 124 + (i * 4);
                        let species =
                            u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                        slots.push(Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: table.get_form_entry(species as usize, 0),
                        });
                    }
                    encounters.push(EncounterArea4 {
                        location,
                        rate: good,
                        encounter,
                        pokemon: slots,
                    });
                }
            }
            Encounter::SuperRod => {
                if srod != 0 {
                    let mut slots = Vec::with_capacity(5);
                    for i in 0..5 {
                        let min = entry[143 + (i * 4)];
                        let max = entry[144 + (i * 4)];
                        let index = 145 + (i * 4);
                        let species =
                            u16::from_le_bytes((&entry[index..(index + 2)]).try_into().unwrap());
                        slots.push(Slot {
                            min_level: min,
                            max_level: max,
                            species,
                            info: table.get_form_entry(species as usize, 0),
                        });
                    }
                    encounters.push(EncounterArea4 {
                        location,
                        rate: srod,
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

pub fn get_encounters(
    encounter: Encounter,
    modifier: usize,
    profile: &Profile4,
) -> Vec<EncounterArea4> {
    let version = profile.version;
    let table: &PersonalTable<PersonalInfoG4> = match version {
        GameVersion::D | GameVersion::P => &personal_table::DP,
        GameVersion::Pt => &personal_table::PT,
        GameVersion::HG | GameVersion::SS => &personal_table::HGSS,
        _ => &personal_table::HGSS,
    };

    if version == GameVersion::D || version == GameVersion::P || version == GameVersion::Pt {
        get_dppt(version, encounter, profile, table, modifier)
    } else {
        get_hgss(version, encounter, table, profile, modifier)
    }
}
