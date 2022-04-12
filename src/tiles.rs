use std::{path::Path, time::Instant};

use image::{io::Reader as ImageReader, RgbaImage};
use std::collections::HashMap;

pub type TileId = u16;

#[derive(Deserialize, Debug)]
pub struct TileMap {
  pub tiles: Vec<Vec<Vec<TileId>>>,

  pub width: u32,
  pub height: u32,
}

// Tilesets

pub struct Tileset {
  pub tiles: HashMap<TileId, RgbaImage>,
  pub width: u32,
  pub height: u32,
}

pub struct TilesetManager {
  pub tilesets: HashMap<String, Tileset>,
}

impl TilesetManager {
  pub fn new() -> Self {
    return TilesetManager {
      tilesets: HashMap::new(),
    };
  }

  pub fn load(mut self: Self, name: String, directory: &str, width: u32, height: u32) -> Self {
    let tileset: &mut Tileset = self.tilesets.entry(name).or_insert(Tileset {
      tiles: HashMap::new(),
      width,
      height,
    });

    let path: &Path = Path::new(directory);

    assert!(path.exists(), "Directory not found: {}", directory);

    let start: Instant = Instant::now();
    let mut i: TileId = 0;

    loop {
      let image = match ImageReader::open(format!("{}/{}.png", directory, i)) {
        Ok(file) => file.with_guessed_format().unwrap().decode(),
        Err(_) => break,
      };

      match image {
        Ok(image) => {
          tileset.tiles.entry(i).or_insert(image.into_rgba8());

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
