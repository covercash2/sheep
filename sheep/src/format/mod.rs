#[cfg(feature = "amethyst")]
pub mod amethyst;

pub mod named;

use SpriteAnchor;

pub trait Format {
    type Data;

    fn encode(&self, dimensions: (u32, u32), sprites: &[SpriteAnchor]) -> Self::Data;
}
