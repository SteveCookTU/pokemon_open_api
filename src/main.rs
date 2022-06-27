mod core;

use crate::core::game::get_games;
use crate::core::gen3::encounter::get_gen3_encounters;
use crate::core::gen4::encounter::get_gen4_encounters;
use crate::core::item::get_items;
use crate::core::location::get_locations;
use crate::core::pokemon::get_pokemon;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let factory = move || {
        App::new()
            .service(get_gen3_encounters)
            .service(get_gen4_encounters)
            .service(get_games)
            .service(get_locations)
            .service(get_pokemon)
            .service(get_items)
    };
    HttpServer::new(factory)
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
}
