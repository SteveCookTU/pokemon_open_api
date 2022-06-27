use actix_web::{get, web, HttpResponse, Responder};
use pkhex_rs::GameVersion;
use pokemon_open_api::{
    LOCATIONS_DPPT, LOCATIONS_EMERALD, LOCATIONS_FRLG, LOCATIONS_HGSS, LOCATIONS_RS,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LocationsParam {
    game: u8,
}

#[derive(Serialize)]
struct LocationsResponse {
    locations: Vec<Location>,
}

#[derive(Serialize)]
struct Location {
    id: u8,
    name: String,
}

#[get("/locations")]
pub async fn get_locations(param: web::Query<LocationsParam>) -> impl Responder {
    if [1, 2, 3, 4, 5, 7, 8, 10, 11, 12].contains(&param.game) {
        let version: GameVersion = (param.game as usize).into();
        let locations = match version {
            GameVersion::E => LOCATIONS_EMERALD.as_slice(),
            GameVersion::FR | GameVersion::LG => LOCATIONS_FRLG.as_slice(),
            GameVersion::R | GameVersion::S => LOCATIONS_RS.as_slice(),
            GameVersion::D | GameVersion::P | GameVersion::Pt => LOCATIONS_DPPT.as_slice(),
            GameVersion::HG | GameVersion::SS => LOCATIONS_HGSS.as_slice(),
            _ => &[""],
        };

        let mut response = LocationsResponse {
            locations: Vec::with_capacity(locations.len()),
        };

        for (index, location) in locations.iter().enumerate() {
            if *location == "?" {
                continue;
            }
            response.locations.push(Location {
                id: index as u8,
                name: location.to_string(),
            });
        }

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NoContent().await.unwrap()
    }
}
