use crate::tiles::{TileMap, Tileset};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

pub fn create_image(tileset: &Tileset, tilemap: &TileMap) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  let img_width = tileset.width * tilemap.width;
  let img_height = tileset.height * tilemap.height;

  let mut image = RgbaImage::new(img_width, img_height);

  for (i, layer) in tilemap.tiles.iter().enumerate() {
    for (x, column) in layer.iter().enumerate() {
      for (y, tile) in column.iter().enumerate() {
        let tile_image: &DynamicImage = &tileset.tiles[&tile];

        let x_pos = x * tileset.width as usize;
        let y_pos = y * tileset.height as usize;

        for tile_x in 0..tileset.width {
          for tile_y in 0..tileset.height {
            let overlay_x = (x_pos + tile_x as usize) as u32;
            let overlay_y = (y_pos + tile_y as usize) as u32;

            if i == 0 {
              image.put_pixel(overlay_x, overlay_y, tile_image.get_pixel(tile_x, tile_y));
              continue;
            }

            let mut existing_pixel = *image.get_pixel(overlay_x, overlay_y);

            let tile_pixel = tile_image.get_pixel(tile_x, tile_y);

            if tile_pixel[3] == 0 {
              continue;
            }

            existing_pixel.blend(&tile_pixel);

            image.put_pixel(overlay_x, overlay_y, existing_pixel);
          }
        }
      }
    }
  }

  return image;
}
