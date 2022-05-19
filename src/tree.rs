use image::{imageops, RgbaImage};

use crate::{
  render,
  tiles::{TileId, TileMap, Tileset},
  PreloadedImages,
};

fn draw_trunk(tileset: &Tileset, pieces: &Vec<TileId>) -> RgbaImage {
  let tilemap = TileMap {
    tiles: vec![vec![pieces.to_vec()]],

    width: 1,
    height: pieces.len() as u32,
  };

  return render::render_tiles(tileset, &tilemap);
}

pub fn draw_treetop(
  tileset: &Tileset,
  images: &PreloadedImages,
  pieces: &Vec<TileId>,
  background_id: usize,
  y_offset: i64,
) -> RgbaImage {
  let trunk: RgbaImage = draw_trunk(tileset, pieces);
  let mut image: RgbaImage = RgbaImage::new(800, 1040);

  imageops::overlay(&mut image, &images.backgrounds[background_id], 0, 0);

  imageops::overlay(&mut image, &images.treetop, 20, 96 + y_offset);
  imageops::overlay(&mut image, &trunk, 227, 560 + y_offset);

  return image;
}
