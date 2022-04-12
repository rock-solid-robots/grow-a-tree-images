use crate::tiles::{TileMap, Tileset};
use image::{ImageBuffer, Pixel, Rgba, RgbaImage};

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
