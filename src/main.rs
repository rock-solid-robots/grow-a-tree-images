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

use image::{io::Reader, ImageBuffer, Rgba};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use tiles::{TileId, TileMap};
use uuid::Uuid;

use crate::tiles::TilesetManager;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
  domain: String,
  directory: String,
}

struct Images {
  treetop: ImageBuffer<Rgba<u8>, Vec<u8>>,
  background: ImageBuffer<Rgba<u8>, Vec<u8>>,
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
    TilesetManager::new().load(String::from("trees"), "./trees", 400, 96);

  let loaded_bg = match Reader::open("trees/background.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let loaded_treetop = match Reader::open("trees/treetop.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let background = loaded_bg.unwrap().into_rgba8();
  let treetop = loaded_treetop.unwrap().into_rgba8();

  rocket::ignite()
    .mount("/tree", routes![handle_request, generate_treetop])
    .manage(tileset_manager)
    .manage(config)
    .manage(Images {
      treetop: treetop,
      background: background,
    })
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

#[post("/top", format = "json", data = "<data>")]
fn generate_treetop(
  tileset_manager: State<TilesetManager>,
  config: State<Config>,
  images: State<Images>,
  data: Json<TreeRequest>,
) -> JsonValue {
  let tileset = tileset_manager.tilesets.get("trees").unwrap();

  let mut pieces = data.pieces.clone();

  while pieces.len() < 5 {
    println!("{}", pieces.len());
    pieces.push(0);
    println!("{:?}", pieces);
  }

  let tilemap = TileMap {
    tiles: vec![vec![data.pieces.clone()]],

    width: 1,
    height: data.pieces.len() as u32,
  };

  let image = render::render_treetop(
    &render::create_image(tileset, &tilemap),
    &images.treetop,
    &images.background,
  );

  let path = format!("{}/{}-treetop.png", config.directory, data.id);

  image.save(path).unwrap();

  let uuid = Uuid::new_v4();

  let url = format!(
    "{}/{}-treetop.png?cache-breaker={}",
    config.domain, data.id, uuid
  );

  json!({ "status": "ok", "url": url })
}
