# Grow a Tree - Image Server

```rust
struct TreeRequest {
  pub background: String,
  pub pieces: Vec<TileId>,
}
```

Basic image compositing, specify a [background](./src/assets/backgrounds/) and an array of [tree tiles](./src/assets/tiles/) and a corresponding image will be generated.

Defaults to listening on port 9090, but can be changed with a `ROCKET_PORT` env variable.