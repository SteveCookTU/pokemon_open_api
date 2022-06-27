use crate::core::util::get_species_list_from_str;
use actix_web::{get, web, HttpResponse, Responder};
use pkhex_rs::game_strings::{ABILITIES_EN, ITEMS_EN, SPECIES_EN, TYPES_EN};
use pkhex_rs::personal_info_g3::PersonalInfoG3;
use pkhex_rs::personal_info_g4::PersonalInfoG4;
use pkhex_rs::personal_table::PersonalTable;
use pkhex_rs::{personal_table, GameVersion, PersonalInfo};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PokemonParam {
    game: Option<u8>,
    species: Option<String>,
}

#[derive(Serialize)]
struct PokemonResponse {
    pokemon: Vec<Pokemon>,
}

#[derive(Serialize)]
struct Pokemon {
    species: u16,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    held_items: Option<Vec<HeldItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abilities: Option<Vec<Ability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<Vec<Type>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stats: Option<Stats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_rate: Option<usize>,
}

#[derive(Serialize)]
struct HeldItem {
    id: u16,
    item: String,
}

#[derive(Serialize)]
struct Ability {
    id: u16,
    ability: String,
}

#[derive(Serialize)]
struct Type {
    id: u8,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Serialize)]
struct Stats {
    hp: usize,
    atk: usize,
    def: usize,
    spa: usize,
    spd: usize,
    spe: usize,
    total: usize,
}

fn get_pokemon_from_personal(
    table: &PersonalTable<impl PersonalInfo>,
    version: GameVersion,
    species_filter: Option<Vec<u16>>,
) -> PokemonResponse {
    let species_filter = species_filter.unwrap_or_default();

    let mut response = PokemonResponse {
        pokemon: Vec::with_capacity(SPECIES_EN.len()),
    };

    for (species, name) in SPECIES_EN.iter().enumerate().skip(1) {
        if (!species_filter.is_empty() && !species_filter.contains(&(species as u16)))
            || !table.is_species_in_game(species)
        {
            continue;
        }
        let info = table.get_form_entry(species, 0);
        let mut abilities = info.get_abilities();
        abilities.dedup();

        let mut types = vec![info.get_type_1(), info.get_type_2()];

        if (version as u8) < 6 {
            types = types
                .into_iter()
                .map(|t| if t >= 9 { t - 1 } else { t })
                .collect();
        }

        types.dedup();

        response.pokemon.push(Pokemon {
            species: species as u16,
            name: name.to_string(),
            held_items: Some(
                info.get_items()
                    .into_iter()
                    .map(|i| HeldItem {
                        id: i as u16,
                        item: ITEMS_EN[i].to_string(),
                    })
                    .collect(),
            ),
            abilities: Some(
                abilities
                    .into_iter()
                    .map(|i| Ability {
                        id: i as u16,
                        ability: ABILITIES_EN[i].to_string(),
                    })
                    .collect(),
            ),
            types: Some(
                types
                    .into_iter()
                    .map(|t| Type {
                        id: t as u8,
                        r#type: TYPES_EN[t].to_string(),
                    })
                    .collect(),
            ),

            stats: Some(Stats {
                hp: info.get_hp(),
                atk: info.get_atk(),
                def: info.get_def(),
                spd: info.get_spd(),
                spa: info.get_spa(),
                spe: info.get_spe(),
                total: info.bst(),
            }),
            capture_rate: Some(info.get_catch_rate()),
        });
    }

    response
}

#[get("/pokemon")]
pub async fn get_pokemon(param: web::Query<PokemonParam>) -> impl Responder {
    let species = param.species.as_ref().map(|s| get_species_list_from_str(s));

    let version: Option<GameVersion> = if let Some(game) = param.game {
        Some((game as usize).into())
    } else {
        None
    };

    let response = if let Some(version) = version {
        if (version as u8) < 6 {
            let table: &PersonalTable<PersonalInfoG3> = match version {
                GameVersion::LG => &personal_table::LG,
                GameVersion::E => &personal_table::E,
                GameVersion::R | GameVersion::S => &personal_table::RS,
                _ => &personal_table::FR,
            };
            get_pokemon_from_personal(table, version, species)
        } else if [7, 8, 10, 11, 12].contains(&(version as u8)) {
            let table: &PersonalTable<PersonalInfoG4> = match version {
                GameVersion::D | GameVersion::P => &personal_table::DP,
                GameVersion::HG | GameVersion::SS => &personal_table::HGSS,
                _ => &personal_table::PT,
            };
            get_pokemon_from_personal(table, version, species)
        } else {
            return HttpResponse::NoContent().await.unwrap();
        }
    } else {
        let mut response = PokemonResponse {
            pokemon: Vec::with_capacity(SPECIES_EN.len()),
        };

        for (species, name) in SPECIES_EN.iter().enumerate().skip(1) {
            response.pokemon.push(Pokemon {
                species: species as u16,
                name: name.to_string(),
                held_items: None,
                abilities: None,
                types: None,
                stats: None,
                capture_rate: None,
            });
        }
        response
    };

    HttpResponse::Ok().json(response)
}
