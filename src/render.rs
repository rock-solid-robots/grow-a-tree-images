use crate::tiles::{TileMap, Tileset};
use image::{imageops, io::Reader as ImageReader, ImageBuffer, Pixel, Rgba, RgbaImage};

pub fn create_image(tileset: &Tileset, tilemap: &TileMap) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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

pub fn render_treetop(trunk: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let loaded_tree_base = match ImageReader::open("trees/base.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let loaded_tree_top = match ImageReader::open("trees/top.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let tree_base = loaded_tree_base.unwrap().into_rgba8();
  let tree_top = loaded_tree_top.unwrap().into_rgba8();

  let mut image = RgbaImage::new(160, 288);

  imageops::overlay(&mut image, &tree_top, 0, 0);
  imageops::overlay(&mut image, &trunk, 64, 128);
  imageops::overlay(&mut image, &tree_base, 0, 224);

  return image;
}

pub fn render_sapling(size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let loaded_sapling1 = match ImageReader::open("saplings/sapling1.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let loaded_sapling2 = match ImageReader::open("saplings/sapling2.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let loaded_sapling3 = match ImageReader::open("saplings/sapling3.png") {
    Ok(file) => file.with_guessed_format().unwrap().decode(),
    Err(_) => std::process::exit(0),
  };

  let sapling1 = loaded_sapling1.unwrap().into_rgba8();
  let sapling2 = loaded_sapling2.unwrap().into_rgba8();
  let sapling3 = loaded_sapling3.unwrap().into_rgba8();

  match size {
    1 => {
      let mut image = RgbaImage::new(32, 32);
      imageops::overlay(&mut image, &sapling1, 0, 0);

      return image;
    }
    2 => {
      let mut image = RgbaImage::new(64, 64);
      imageops::overlay(&mut image, &sapling2, 0, 0);

      return image;
    }
    3 => {
      let mut image = RgbaImage::new(128, 128);
      imageops::overlay(&mut image, &sapling3, 0, 0);

      return image;
    }
    _ => {
      println!("Invalid sapling size");
      std::process::exit(0);
    }
  };
}
