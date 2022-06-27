use actix_web::{get, HttpResponse, Responder};
use pkhex_rs::game_strings::ITEMS_EN;
use serde::Serialize;

#[derive(Serialize)]
struct ItemsResponse {
    items: Vec<Item>,
}

#[derive(Serialize)]
struct Item {
    id: u16,
    name: String,
}

#[get("/items")]
pub async fn get_items() -> impl Responder {
    let mut response = ItemsResponse {
        items: Vec::with_capacity(ITEMS_EN.len()),
    };

    for (i, item) in ITEMS_EN.iter().enumerate().skip(1) {
        if item.is_empty() {
            continue;
        }
        response.items.push(Item {
            id: i as u16,
            name: item.to_string(),
        });
    }

    HttpResponse::Ok().json(response)
}
