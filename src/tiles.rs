use std::{path::Path, time::Instant};

use image::io::Reader as ImageReader;
use std::collections::HashMap;

use image::DynamicImage;

#[derive(Clone)]
pub struct TileRefMap {
  pub tiles: Vec<Vec<TileRef>>,
}

#[derive(Clone)]
pub struct TileRef {
  pub id: u32,

  pub overlay: bool,
}

pub struct Tile {
  pub id: u32,

  pub image: DynamicImage,
}

pub struct TileManager {
  tilesets: HashMap<String, Vec<Tile>>,
}

impl TileManager {
  pub fn new() -> Self {
    let tilesets: HashMap<String, Vec<Tile>> = HashMap::new();

    return TileManager { tilesets };
  }

  pub fn load(mut self: Self, name: String, directory: &str) -> Self {
    let tiles = self.tilesets.entry(name).or_insert(Vec::new());

    let path: &Path = Path::new(directory);

    if !path.exists() {
      panic!("Directory not found: {}", directory);
    }

    let start = Instant::now();
    let mut i = 0;

    loop {
      let image = match ImageReader::open(path) {
        Ok(file) => file.with_guessed_format().unwrap().decode(),
        Err(_) => break,
      };

      match image {
        Ok(image) => {
          tiles.push(Tile {
            id: i,
            image: image,
          });

          println!("Loaded tile: {}/{}.png", directory, i);
        }
        Err(_) => panic!("Could not decode image tile: {}/{}.png", directory, i),
      }

      i += 1;
    }

    println!("Loaded {} tiles in {:?}", i, start.elapsed());

    return self;
  }
}
