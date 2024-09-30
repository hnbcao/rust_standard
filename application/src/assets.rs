use rust_embed::RustEmbed;

pub const COMMIT: &str = include_str!("../assets/commit");

#[derive(RustEmbed)]
#[folder = "assets/openapi"]
struct AssetsOpenapi;

#[derive(RustEmbed)]
#[folder = "assets/static"]
struct AssetsStatic;