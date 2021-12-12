use ggez::{graphics, mint::Vector2};

pub struct SpriteSheet {
    pub tile_size: Vector2<i32>,
    pub tile_ratio: Vector2<f32>,
    pub atlas: graphics::Image,
}

pub struct SpriteSheetConfig {
    pub tile_size_x: i32,
    pub tile_size_y: i32,
    pub atlas_path: String,
}