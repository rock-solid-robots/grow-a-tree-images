use std::time::Instant;

use image::{GenericImage, RgbaImage};
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use uuid::Uuid;

use crate::tiles::{Tile, TileRef, TileRefMap};

#[derive(Deserialize, Debug)]
pub struct ImageRequest {
  pub user_id: String,
  pub bakery: Bakery,

  pub overlay: Option<PlacedFurniture>,
}

#[derive(Deserialize, Debug)]
pub struct Bakery {
  pub tiles: Vec<Vec<u32>>,
  pub furniture: Vec<PlacedFurniture>,
}

#[derive(Deserialize, Debug)]
pub struct PlacedFurniture {
  pub position: [u32; 2],

  pub horizontal: bool,

  pub item: Furniture,
}

#[derive(Deserialize, Debug)]
pub struct Furniture {
  // name: String,
  // description: String,

  // price: u32,
  pub tiles: Vec<Vec<u32>>,
}

#[post("/", format = "json", data = "<data>")]
pub fn create_image(tiles: State<Vec<Tile>>, data: Json<ImageRequest>) -> JsonValue {
  let mut tilemap = TileRefMap { tiles: Vec::new() };
  let bakery = &data.bakery;

  let start = Instant::now();

  println!("{:?}", data);

  let bakery_width = bakery.tiles.len();
  let bakery_height = bakery.tiles[0].len();

  println!("{}x{}", bakery_width, bakery_height);

  for x in 0..bakery_width {
    let mut column: Vec<TileRef> = Vec::new();

    for y in 0..bakery_height {
      let tile_id = bakery.tiles[x][y];

      let tile = TileRef {
        id: tile_id,
        overlay: false,
      };

      column.push(tile);
    }

    tilemap.tiles.push(column);
  }

  println!(
    "Creating image of size ({}, {})",
    32 * bakery_width,
    32 * bakery_height
  );

  let mut img = RgbaImage::new((32 * bakery_width) as u32, (32 * bakery_height) as u32);

  for x in 0..tilemap.tiles.len() {
    for y in 0..tilemap.tiles[x].len() {
      let tile_id = tilemap.tiles[x][y].id;
      let tile = &tiles[tile_id as usize];

      println!(
        "Setting tile at ({}, {}) - ({}, {}) to #{}",
        x,
        y,
        x as u32 * 32,
        y as u32 * 32,
        tile_id
      );

      match img.copy_from(&tile.image, x as u32 * 32, y as u32 * 32) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
      }
    }
  }

  let mut furniture_tilemap = tilemap.clone();

  for placed_furniture in &bakery.furniture {
    let starting_x = placed_furniture.position[0];
    let starting_y = placed_furniture.position[1];

    let mut width = placed_furniture.item.tiles.len() as u32;
    let mut height = placed_furniture.item.tiles[0].len() as u32;

    if placed_furniture.horizontal == false {
      std::mem::swap(&mut width, &mut height);
    }

    for x in starting_x..starting_x + width {
      for y in starting_y..starting_y + height {
        let x_offset = x - starting_x;
        let y_offset = y - starting_y;

        let tile_id = placed_furniture.item.tiles[x_offset as usize][y_offset as usize];
        let tile = &tiles[tile_id as usize];

        println!("Placing furniture tile #{} at ({}, {})", tile_id, x, y);
        furniture_tilemap.tiles[x as usize][y as usize].id = tile.id;
      }
    }
  }

  match &data.overlay {
    Some(overlay) => {
      let starting_x = overlay.position[0];
      let starting_y = overlay.position[1];

      let mut width = overlay.item.tiles.len() as u32;
      let mut height = overlay.item.tiles[0].len() as u32;

      if overlay.horizontal == false {
        std::mem::swap(&mut width, &mut height);
      }

      for x in starting_x..starting_x + width {
        for y in starting_y..starting_y + height {
          let x_offset = x - starting_x;
          let y_offset = y - starting_y;

          let tile_id = overlay.item.tiles[x_offset as usize][y_offset as usize];
          let tile = &tiles[tile_id as usize];

          println!(
            "Placing overlayed furniture tile #{} at ({}, {})",
            tile_id, x, y
          );
          furniture_tilemap.tiles[x as usize][y as usize] = TileRef {
            id: tile.id,
            overlay: true,
          }
        }
      }
    }
    None => {}
  }

  for x in 0..furniture_tilemap.tiles.len() {
    for y in 0..furniture_tilemap.tiles[x].len() {
      let tile_id = furniture_tilemap.tiles[x][y].id;
      let tile = &tiles[tile_id as usize];

      if tile_id != tilemap.tiles[x][y].id {
        println!(
          "Overlaying tile at ({}, {}) - ({}, {}) to #{}",
          x,
          y,
          x as u32 * 32,
          y as u32 * 32,
          tile_id
        );

        image::imageops::overlay(
          &mut img,
          &tile.image,
          (x as u32 * 32) as i64,
          (y as u32 * 32) as i64,
        );
      }

      if furniture_tilemap.tiles[x][y].overlay {
        let overlay_tile = &tiles[0];

        println!(
          "Overlaying tile at ({}, {}) - ({}, {}) with #{}",
          x,
          y,
          x as u32 * 32,
          y as u32 * 32,
          tile_id
        );

        image::imageops::overlay(
          &mut img,
          &overlay_tile.image,
          (x as u32 * 32) as i64,
          (y as u32 * 32) as i64,
        )
      }
    }
  }

  let path = format!("/var/www/tiling-images/{}.png", data.user_id);

  img.save(path).unwrap();

  println!("Created image in {:?}", start.elapsed());

  let my_uuid = Uuid::new_v4();

  let url = format!(
    "https://cdn.milesmoonlove.com/{}.png?{}",
    data.user_id, my_uuid
  );
  json!({ "status": "ok", "url": url })
}
