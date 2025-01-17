use rust_embed::RustEmbed;

pub const COMMIT: &str = include_str!("../assets/commit");

#[derive(RustEmbed)]
#[folder = "assets/openapi"]
pub struct AssetsOpenapi;

#[derive(RustEmbed)]
#[folder = "assets/static"]
pub struct AssetsStatic;