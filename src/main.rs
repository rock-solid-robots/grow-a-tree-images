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
use tiles::{TileId, TileMap};

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

  let url = format!("{}/{}.png", config.domain, data.id);

  json!({ "status": "ok", "url": url })
}

#[derive(Deserialize)]
struct TreeRequest {
  pub id: String,

  pub pieces: Vec<TileId>,
}

#[post("/tree", format = "json", data = "<data>")]
fn render_partial_tree(
  tileset_manager: State<TilesetManager>,
  config: State<Config>,
  data: Json<TreeRequest>,
) -> JsonValue {
  let tileset = tileset_manager.tilesets.get("trees").unwrap();

  let tilemap = TileMap {
    tiles: vec![vec![data.pieces.clone()]],

    width: 1,
    height: data.pieces.len() as u32,
  };

  let image = match data.pieces.len() > 3 {
    true => render::render_treetop(render::create_image(tileset, &tilemap)),
    false => render::render_sapling(data.pieces.len() as u32),
  };

  let path = format!("{}/{}-treetop.png", config.directory, data.id);

  image.save(path).unwrap();

  let url = format!("{}/{}-treetop.png", config.domain, data.id);

  json!({ "status": "ok", "url": url })
}
