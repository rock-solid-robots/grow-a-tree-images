use image::{imageops, RgbaImage};

use crate::{
  tiles::{self, TileId, TileMap, Tileset},
  PreloadedImages,
};

fn draw_trunk(tileset: &Tileset, pieces: &Vec<TileId>) -> RgbaImage {
  let tilemap = TileMap {
    tiles: vec![vec![pieces.to_vec()]],

    width: 1,
    height: pieces.len() as u32,
  };

  return tiles::render(tileset, &tilemap);
}

pub fn draw_treetop(
  tileset: &Tileset,
  images: &PreloadedImages,
  pieces: &Vec<TileId>,
  background_id: usize,
) -> RgbaImage {
  let trunk: RgbaImage = draw_trunk(tileset, pieces);
  let mut image: RgbaImage = RgbaImage::new(800, 1040);

  let mut y_offset = if background_id == 0 { -74 } else { 0 };

  if pieces.len() < 5 {
    y_offset += ((5 - pieces.len()) * 64) as i64;
  }

  imageops::overlay(&mut image, &images.backgrounds[background_id], 0, 0);

  imageops::overlay(&mut image, &images.treetop, 20, 96 + y_offset);
  imageops::overlay(&mut image, &trunk, 227, 560 + y_offset);

  return image;
}
