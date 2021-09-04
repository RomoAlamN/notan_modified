use crate::assets::Loader;
use crate::graphics::Graphics;
// use crate::graphics::{Texture, TextureInfo};
use notan_graphics::{Texture, TextureInfo};

pub fn create_texture_parser() -> Loader {
    Loader::new()
        .use_parser(parse_image)
        .from_extensions(&["png"])
}

fn parse_image(id: &str, data: Vec<u8>, gfx: &mut Graphics) -> Result<Texture, String> {
    let texture = gfx.create_texture().from_image(&data).build()?;
    notan_log::debug!("Asset '{}' parsed as Texture", id);
    Ok(texture)
}
