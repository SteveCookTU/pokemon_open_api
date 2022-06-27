use crate::core::util::{get_location_list_from_str, get_species_list_from_str};
use actix_web::{get, web, HttpResponse, Responder};
use pkhex_rs::game_strings::SPECIES_EN;
use pkhex_rs::GameVersion;
use pokemon_open_api::encounter::Encounter as EncounterT;
use pokemon_open_api::encounter_area::EncounterArea;
use pokemon_open_api::{gen3, LOCATIONS_EMERALD, LOCATIONS_FRLG, LOCATIONS_RS};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncountersParam {
    game: u8,
    species: Option<String>,
    locations: Option<String>,
}

#[derive(Serialize)]
struct EncountersResponse {
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

#[get("/encounters/gen3")]
pub async fn get_gen3_encounters(param: web::Query<EncountersParam>) -> impl Responder {
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

    if param.game < 6 {
        HttpResponse::Ok().json(get_encounters(param.game, species, locations))
    } else {
        HttpResponse::NoContent().await.unwrap()
    }
}

fn get_encounters(game: u8, species: Vec<u16>, locations: Vec<u8>) -> EncountersResponse {
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
        let encounter_areas = gen3::get_encounters(encounter_type, version);
        for encounter_area in encounter_areas {
            if !locations.is_empty() && !locations.contains(&encounter_area.get_location()) {
                continue;
            }
            let mut area = EncounterAreaResp {
                location_id: encounter_area.get_location(),
                location_name: match version {
                    GameVersion::E => {
                        LOCATIONS_EMERALD[encounter_area.get_location() as usize].to_string()
                    }
                    GameVersion::FR | GameVersion::LG => {
                        LOCATIONS_FRLG[encounter_area.get_location() as usize].to_string()
                    }
                    GameVersion::R | GameVersion::S => {
                        LOCATIONS_RS[encounter_area.get_location() as usize].to_string()
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

    EncountersResponse { encounters }
}
