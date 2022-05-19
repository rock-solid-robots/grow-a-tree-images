use std::{path::Path, time::Instant};

use image::{io::Reader as ImageReader, Pixel, RgbaImage};
use std::collections::HashMap;

pub type TileId = u16;

#[derive(Deserialize, Debug)]
pub struct TileMap {
  pub tiles: Vec<Vec<Vec<TileId>>>,

  pub width: u32,
  pub height: u32,
}

pub fn render(tileset: &Tileset, tilemap: &TileMap) -> RgbaImage {
  let img_width = tileset.width * tilemap.width;
  let img_height = tileset.height * tilemap.height;

  let mut image = RgbaImage::new(img_width, img_height);

  for (i, layer) in tilemap.tiles.iter().enumerate() {
    for (x, column) in layer.iter().enumerate() {
      for (y, tile) in column.iter().enumerate() {
        let tile_image: &RgbaImage = &tileset.tiles[&tile];

        let x_pos = x * tileset.width as usize;
        let y_pos = y * tileset.height as usize;

        for (tile_x, tile_y, pixel_tile) in tile_image.enumerate_pixels() {
          let overlay_x = (x_pos + tile_x as usize) as u32;
          let overlay_y = (y_pos + tile_y as usize) as u32;

          if i == 0 {
            image.put_pixel(overlay_x, overlay_y, *pixel_tile);
            continue;
          }

          if pixel_tile[3] == 0 {
            continue;
          }

          image.get_pixel_mut(overlay_x, overlay_y).blend(pixel_tile);
        }
      }
    }
  }

  return image;
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

  pub fn load(mut self: Self, name: &str, directory: &str, width: u32, height: u32) -> Self {
    let tileset: &mut Tileset = self.tilesets.entry(name.to_string()).or_insert(Tileset {
      tiles: HashMap::new(),
      width,
      height,
    });

    let path: &Path = Path::new(&directory);

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
