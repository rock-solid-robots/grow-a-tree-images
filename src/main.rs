#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod render;
mod tiles;
use std::fs;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use tiles::TileMap;
use uuid::Uuid;

use crate::tiles::TilesetManager;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
  domain: String,
  directory: String,
}

fn main() {
  let contents = match fs::read_to_string("Config.toml") {
    Ok(c) => c,
    Err(_) => {
      panic!("Could not read config file.");
    }
  };
  let config: Config = toml::from_str(&contents).unwrap();

  let tileset_manager: TilesetManager =
    TilesetManager::new().load(String::from("bakery"), "./bakery", 32, 32);

  rocket::ignite()
    .mount("/", routes![handle_request])
    .manage(tileset_manager)
    .manage(config)
    .launch();
}

#[derive(Deserialize)]
struct ImageRequest {
  pub id: String,

  pub tileset: String,
  pub tiles: TileMap,
}

#[post("/", format = "json", data = "<data>")]
fn handle_request(
  tileset_manager: State<TilesetManager>,
  config: State<Config>,
  data: Json<ImageRequest>,
) -> JsonValue {
  let tileset = tileset_manager.tilesets.get(&data.tileset).unwrap();

  let image = render::create_image(tileset, &data.tiles);

  let path = format!("{}/{}.png", config.directory, data.id);

  image.save(path).unwrap();

  let uuid = Uuid::new_v4();

  let url = format!("{}/{}.png?{}", config.domain, data.id, uuid);

  json!({ "status": "ok", "url": url })
}
