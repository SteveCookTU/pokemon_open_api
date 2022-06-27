use crate::core::util::{get_location_list_from_str, get_species_list_from_str};
use actix_web::{get, web, HttpResponse, Responder};
use pkhex_rs::game_strings::SPECIES_EN;
use pkhex_rs::GameVersion;
use pokemon_open_api::encounter::Encounter as EncounterT;
use pokemon_open_api::encounter_area::EncounterArea;
use pokemon_open_api::gen4::Profile4;
use pokemon_open_api::{gen4, LOCATIONS_DPPT, LOCATIONS_HGSS};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncountersParam {
    game: u8,
    species: Option<String>,
    locations: Option<String>,
    radar: Option<bool>,
    swarm: Option<bool>,
    dual: Option<u8>,
    radio: Option<usize>,
    time: Option<usize>,
    day: Option<usize>,
    national: Option<bool>,
}

#[derive(Serialize)]
struct EncountersResponse {
    swarm: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    radio: Option<Radio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dual: Option<DualSlot>,
    time: Time,
    #[serde(skip_serializing_if = "Option::is_none")]
    national_dex: Option<bool>,
    encounters: Vec<EncounterAreaResp>,
}

#[derive(Serialize)]
struct EncounterAreaResp {
    location_id: u8,
    location_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate: Option<u8>,
    encounter_type: String,
    pokemon: Vec<Slot>,
}

#[derive(Serialize)]
struct Slot {
    slot_num: u8,
    species: u16,
    species_name: String,
    min_level: u8,
    max_level: u8,
}

#[derive(Serialize)]
struct Radio {
    id: u8,
    sound: String,
}

impl Into<Radio> for u8 {
    fn into(self) -> Radio {
        let sound = match self {
            0 => "None",
            1 => "Hoenn Sound",
            2 => "Sinnoh Sound",
            _ => "",
        };
        Radio {
            id: self,
            sound: sound.to_string(),
        }
    }
}

#[derive(Serialize)]
struct DualSlot {
    game_id: u8,
    title: String,
}

impl Into<DualSlot> for u8 {
    fn into(self) -> DualSlot {
        let title = match self {
            1 => "Sapphire",
            2 => "Ruby",
            3 => "Emerald",
            4 => "FireRed",
            5 => "LeafGreen",
            _ => "None",
        };

        DualSlot {
            game_id: self,
            title: title.to_string(),
        }
    }
}

#[derive(Serialize)]
struct Time {
    id: u8,
    time: String,
}

impl Into<Time> for u8 {
    fn into(self) -> Time {
        let time = match self {
            0 => "Morning",
            1 => "Day",
            2 => "Night",
            _ => "",
        };
        Time {
            id: self,
            time: time.to_string(),
        }
    }
}

#[get("/encounters/gen4")]
pub async fn get_gen4_encounters(param: web::Query<EncountersParam>) -> impl Responder {
    let species = if let Some(species_list) = &param.species {
        get_species_list_from_str(species_list)
    } else {
        vec![]
    };

    let locations = if let Some(location_list) = &param.locations {
        get_location_list_from_str(location_list)
    } else {
        vec![]
    };

    let version: GameVersion = (param.game as usize).into();

    let profile = Profile4 {
        version,
        has_national_dex: param.national.unwrap_or_default(),
        radio: param.radio.unwrap_or_default(),
        swarm: param.swarm.unwrap_or_default(),
        radar: param.radar.unwrap_or_default(),
        dual: param
            .dual
            .map_or(GameVersion::Unknown, |v| (v as usize).into()),
    };

    if [7, 8, 10, 11, 12].contains(&param.game) {
        HttpResponse::Ok().json(get_encounters(
            param.game,
            species,
            locations,
            profile,
            param.time.unwrap_or_default(),
            param.day.unwrap_or_default(),
        ))
    } else {
        HttpResponse::NoContent().await.unwrap()
    }
}

fn get_encounters(
    game: u8,
    species: Vec<u16>,
    locations: Vec<u8>,
    profile: Profile4,
    time: usize,
    day: usize,
) -> EncountersResponse {
    let mut encounters = Vec::new();
    let version: GameVersion = (game as usize).into();
    let encounter_types = vec![
        EncounterT::Grass,
        EncounterT::RockSmash,
        EncounterT::OldRod,
        EncounterT::GoodRod,
        EncounterT::SuperRod,
        EncounterT::Surfing,
    ];

    for encounter_type in encounter_types {
        let encounter_areas = if encounter_type == EncounterT::BugCatchingContest {
            gen4::get_encounters(encounter_type, day, &profile)
        } else {
            gen4::get_encounters(encounter_type, time, &profile)
        };
        for encounter_area in encounter_areas {
            if !locations.is_empty() && !locations.contains(&encounter_area.get_location()) {
                continue;
            }
            let mut area = EncounterAreaResp {
                location_id: encounter_area.get_location(),
                location_name: match version {
                    GameVersion::P | GameVersion::D | GameVersion::Pt => {
                        LOCATIONS_DPPT[encounter_area.get_location() as usize].to_string()
                    }
                    GameVersion::HG | GameVersion::SS => {
                        LOCATIONS_HGSS[encounter_area.get_location() as usize].to_string()
                    }
                    _ => "".to_string(),
                },
                rate: if encounter_area.get_rate() == 0 {
                    None
                } else {
                    Some(encounter_area.get_rate())
                },
                encounter_type: encounter_area.get_encounter().to_string(),
                pokemon: vec![],
            };
            let mut slot_num = 0;
            for pokemon in encounter_area.get_pokemon() {
                if !species.is_empty() && !species.contains(&pokemon.get_species()) {
                    slot_num += 1;
                    continue;
                }
                area.pokemon.push(Slot {
                    slot_num,
                    species: pokemon.get_species(),
                    species_name: SPECIES_EN[pokemon.get_species() as usize].to_string(),
                    min_level: pokemon.get_min_level(),
                    max_level: pokemon.get_max_level(),
                });
                slot_num += 1;
            }
            if !area.pokemon.is_empty() {
                encounters.push(area);
            }
        }
    }

    encounters.sort_by(|e1, e2| e1.location_id.cmp(&e2.location_id));

    if profile.version == GameVersion::D
        || profile.version == GameVersion::P
        || profile.version == GameVersion::Pt
    {
        EncountersResponse {
            swarm: profile.swarm,
            radio: None,
            radar: Some(profile.radar),
            dual: Some((profile.dual as u8).into()),
            time: (time as u8).into(),
            national_dex: None,
            encounters,
        }
    } else {
        EncountersResponse {
            swarm: profile.swarm,
            radio: Some((profile.radio as u8).into()),
            radar: None,
            dual: None,
            time: (time as u8).into(),
            national_dex: Some(profile.has_national_dex),
            encounters,
        }
    }
}
