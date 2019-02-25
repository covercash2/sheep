use super::Format;
use SpriteAnchor;

pub struct NamedFormat {
    names: Vec<String>,
}

impl NamedFormat {
    pub fn new(names: Vec<String>) -> NamedFormat {
        return NamedFormat { names: names };
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedSprite {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub offsets: Option<[f32; 2]>,
}

impl NamedSprite {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl From<&SpriteAnchor> for NamedSprite {
    fn from(anchor: &SpriteAnchor) -> NamedSprite {
        return NamedSprite {
            name: String::new(),
            x: anchor.position.0 as f32,
            y: anchor.position.1 as f32,
            width: anchor.dimensions.0 as f32,
            height: anchor.dimensions.1 as f32,
            offsets: None,
        };
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializedNamedSpriteSheet {
    pub spritesheet_width: f32,
    pub spritesheet_height: f32,
    pub sprites: Vec<NamedSprite>,
}

impl Format for NamedFormat {
    type Data = SerializedNamedSpriteSheet;

    fn encode(&self, dimensions: (u32, u32), sprites: &[SpriteAnchor]) -> Self::Data {
        let sprite_positions = sprites
            .iter()
            .map(|it| {
                let id = it.id;
                let mut sprite: NamedSprite = it.into();
                sprite.set_name(self.names[id].clone());
                return sprite;
            })
            .collect::<Vec<NamedSprite>>();

        SerializedNamedSpriteSheet {
            spritesheet_width: dimensions.0 as f32,
            spritesheet_height: dimensions.1 as f32,
            sprites: sprite_positions,
        }
    }
}
