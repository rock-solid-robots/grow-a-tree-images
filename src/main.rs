#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod render;
mod tiles;
mod tree;

use std::fs;

use image::{io::Reader, RgbaImage};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use tiles::TileId;
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
    TilesetManager::new().load("trees", "./assets/tiles/", 400, 96);

  rocket::ignite()
    .mount("/tree", routes![generate_treetop])
    .manage(tileset_manager)
    .manage(config)
    .manage(preload_images())
    .launch();
}

#[derive(Deserialize)]
struct TreeRequest {
  pub id: String,

  pub background_id: usize,
  pub size: u32,

  pub pieces: Vec<TileId>,
}

#[post("/", format = "json", data = "<data>")]
fn generate_treetop(
  config: State<Config>,
  tileset_manager: State<TilesetManager>,
  images: State<PreloadedImages>,
  data: Json<TreeRequest>,
) -> JsonValue {
  let background_id = data.background_id;
  let y_offset = if data.size <= 5 { -74 } else { 0 };

  let image = tree::draw_treetop(
    &tileset_manager.tilesets.get("trees").unwrap(),
    &images,
    &data.pieces,
    background_id,
    y_offset,
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

pub struct PreloadedImages {
  treetop: RgbaImage,
  backgrounds: Vec<RgbaImage>,
}

fn preload_images() -> PreloadedImages {
  let loaded_treetop = match Reader::open("./assets/treetop.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let treetop = loaded_treetop.unwrap().into_rgba8();

  let mut backgrounds: Vec<RgbaImage> = vec![];

  for image in fs::read_dir("./assets/backgrounds").unwrap() {
    println!("{:?}", image);

    let loaded_image = match Reader::open("trees/background.png") {
      Ok(file) => file.with_guessed_format().unwrap().decode(),
      Err(_) => {
        println!(
          "Failed to load image: ./assets/backgrounds/{}",
          image.unwrap().path().display()
        );
        continue;
      }
    };

    backgrounds.push(loaded_image.unwrap().into_rgba8())
  }

  return PreloadedImages {
    treetop,
    backgrounds,
  };
}
