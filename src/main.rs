#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod render;
mod tiles;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use tiles::TileMap;

use crate::tiles::TilesetManager;

fn main() {
  let tileset_manager: TilesetManager =
    TilesetManager::new().load(String::from("bakery"), "./bakery", 32, 32);

  rocket::ignite()
    .mount("/", routes![handle_request])
    .manage(tileset_manager)
    .launch();
}

#[derive(Deserialize)]
struct ImageRequest {
  pub id: String,

  pub tileset: String,
  pub tiles: TileMap,
}

#[post("/", format = "json", data = "<data>")]
fn handle_request(tileset_manager: State<TilesetManager>, data: Json<ImageRequest>) -> JsonValue {
  let tileset = tileset_manager.tilesets.get(&data.tileset).unwrap();

  let url = render::create_image(tileset, &data.tiles, &data.id);

  json!({ "status": "ok", "url": url })
}
