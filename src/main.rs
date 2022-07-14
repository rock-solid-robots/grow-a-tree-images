#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod tiles;
mod tree;

use std::{collections::HashMap, fs, io::Cursor};

use image::{io::Reader, ImageOutputFormat, RgbaImage};
use rocket::{State, response::{Response, Responder}, http::{Status, ContentType}};
use rocket_contrib::json::Json;
use tiles::TileId;

use crate::tiles::TilesetManager;

use serde_derive::Deserialize;

fn main() {
  let backgrounds = preload_images();

  let tileset_manager: TilesetManager =
    TilesetManager::new().load("trees", "./src/assets/tiles/", 400, 96);

  rocket::ignite()
    .mount("/tree", routes![generate_treetop])
    .manage(tileset_manager)
    .manage(backgrounds)
    .launch();
}

#[derive(Deserialize)]
struct TreeRequest {
  pub background: String,
  pub pieces: Vec<TileId>,
}

#[post("/", format = "json", data = "<data>")]
fn generate_treetop<'a>(
  tileset_manager: State<TilesetManager>,
  images: State<PreloadedImages>,
  data: Json<TreeRequest>,
) -> Response<'a> {
  let mut buffer = Cursor::new(Vec::new());

  tree::draw_treetop(
    &tileset_manager.tilesets.get("trees").unwrap(),
    &images,
    &data.pieces,
    &data.background,
  )
  .write_to(&mut buffer, ImageOutputFormat::Png)
  .unwrap();
  
  let response = Response::build().status(Status::Ok).header(ContentType::PNG).sized_body(buffer).finalize();

  return response;
}

pub struct PreloadedImages {
  treetop: RgbaImage,
  backgrounds: HashMap<String, RgbaImage>,
}

fn preload_images() -> PreloadedImages {
  let loaded_treetop = match Reader::open("./src/assets/treetop.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let treetop = loaded_treetop.unwrap().into_rgba8();

  let mut backgrounds = HashMap::new();

  for file in fs::read_dir("./src/assets/backgrounds").unwrap() {
    let image = match file {
      Ok(i) => i,
      Err(_) => panic!("Error accessing directory file."),
    };

    println!("Loading background: {:?}", image.path());

    let loaded_image = match Reader::open(image.path()) {
      Ok(file) => file.with_guessed_format().unwrap().decode(),
      Err(_) => panic!("Failed to load image: {}", image.path().display()),
    };

    let name = image.file_name().to_str().unwrap().to_string();

    backgrounds.insert(
      (name[0..name.len() - 4]).to_string(),
      loaded_image.unwrap().into_rgba8(),
    );
  }

  return PreloadedImages {
    treetop,
    backgrounds,
  };
}
