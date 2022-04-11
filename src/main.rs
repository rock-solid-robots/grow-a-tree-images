#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod bakery;
mod tiles;
pub mod util;
use crate::tiles::TileManager;

fn main() {
  let tile_manager: TileManager = TileManager::new().load(String::from("bakery"), "./bakery");

  rocket::ignite()
    .mount("/", routes![bakery::create_image])
    .manage(tile_manager)
    .launch();
}
