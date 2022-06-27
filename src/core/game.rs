use actix_web::{get, HttpResponse, Responder};
use pkhex_rs::GameVersion;
use serde::Serialize;

#[derive(Serialize)]
struct GamesResponse {
    games: Vec<Game>,
}

#[derive(Serialize)]
struct Game {
    id: u8,
    generation: u8,
    title: &'static str,
}

#[get("/games")]
pub async fn get_games() -> impl Responder {
    let response = GamesResponse {
        games: vec![
            Game {
                id: GameVersion::S as u8,
                generation: 3,
                title: "Sapphire",
            },
            Game {
                id: GameVersion::R as u8,
                generation: 3,
                title: "Ruby",
            },
            Game {
                id: GameVersion::E as u8,
                generation: 3,
                title: "Emerald",
            },
            Game {
                id: GameVersion::FR as u8,
                generation: 3,
                title: "FireRed",
            },
            Game {
                id: GameVersion::LG as u8,
                generation: 3,
                title: "LeafGreen",
            },
            Game {
                id: GameVersion::D as u8,
                generation: 4,
                title: "Diamond",
            },
            Game {
                id: GameVersion::P as u8,
                generation: 4,
                title: "Pearl",
            },
            Game {
                id: GameVersion::Pt as u8,
                generation: 4,
                title: "Platinum",
            },
            Game {
                id: GameVersion::HG as u8,
                generation: 4,
                title: "HeartGold",
            },
            Game {
                id: GameVersion::SS as u8,
                generation: 4,
                title: "SoulSilver",
            },
        ],
    };
    HttpResponse::Ok().json(response)
}
