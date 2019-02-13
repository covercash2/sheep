use serde::Serialize;

#[cfg(feature = "amethyst")]
pub mod amethyst;

pub mod named;

use SpriteAnchor;

pub trait Format {
    type Data: Serialize;

    fn encode(&self, dimensions: (u32, u32), sprites: &[SpriteAnchor]) -> Self::Data;
}
