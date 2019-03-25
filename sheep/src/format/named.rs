use super::Format;
use SpriteAnchor;

pub struct AmethystNamedFormat;

/// `NameSprite` represents a field in a `SerializedNamedSpriteSheet`.
/// All of the fields, except `name`, mimic the `SpritePosition` struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedSpritePosition {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub offsets: Option<[f32; 2]>,
}

impl From<&SpriteAnchor> for NamedSpritePosition {
    fn from(anchor: &SpriteAnchor) -> NamedSpritePosition {
        return NamedSpritePosition {
            name: anchor.name.clone(),
            x: anchor.position.0 as f32,
            y: anchor.position.1 as f32,
            width: anchor.dimensions.0 as f32,
            height: anchor.dimensions.1 as f32,
            offsets: None,
        };
    }
}

/// `SerializedNamedSpriteSheet` is the serializable representation of the sprite sheet.
/// It is similar to `SerializedSpriteSheet`, except that it has a `Vec` of `NamedSpritePosition`s
/// instead of `SpriteAnchor`s
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializedNamedSpriteSheet {
    pub spritesheet_width: f32,
    pub spritesheet_height: f32,
    pub sprites: Vec<NamedSpritePosition>,
}

impl Format for AmethystNamedFormat {
    type Data = SerializedNamedSpriteSheet;
    type Options = ();

    fn encode(
        dimensions: (u32, u32),
        sprites: &[SpriteAnchor],
        _options: Self::Options,
    ) -> Self::Data {
        let sprite_positions = sprites
            .iter()
            .map(Into::into)
            .collect::<Vec<NamedSpritePosition>>();

        SerializedNamedSpriteSheet {
            spritesheet_width: dimensions.0 as f32,
            spritesheet_height: dimensions.1 as f32,
            sprites: sprite_positions,
        }
    }
}
