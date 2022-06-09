# Image Tiling Server

Takes in a tilemap on `/`, saves the output image to the directory specified in config, and returns a url pointing there.

```rust
struct ImageRequest {
  pub id: String,

  pub tileset: String,
  pub tiles: TileMap,
}

pub struct TileMap {
  pub tiles: Vec<Vec<Vec<TileId>>>,

  pub width: u32,
  pub height: u32,
}
```

Tile maps are represented as an array of "Layers", each being a 2D array of tiles.

Defaults to listening on port 9090, can be changed with the `ROCKET_PORT` env variable.