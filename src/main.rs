use macroquad::prelude::*;
use macroquad::texture::Texture2D;

struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }
}

struct Item {
    tag: String,
    texture: Texture2D,
    position: Pos,
    flavor_text: String,
    link: Option<Box<Item>>,
}

impl Item {
    fn new(
        tag: &str,
        texture: Texture2D,
        position: Pos,
        flavor_text: &str,
        link: Option<Box<Item>>,
    ) -> Self {
        Item {
            tag: tag.to_owned(),
            texture,
            position,
            flavor_text: flavor_text.to_owned(),
            link,
        }
    }
}

#[macroquad::main("EscapeRoom")]
async fn main() {
    let mut items: Vec<Item> = Vec::new();

    let door_texture: Texture2D = load_texture("assets/ExitDoor.png").await.unwrap();

    let door = Item::new(
        "exit_door",
        door_texture,
        Pos::new(100f32, 20f32),
        "I don't really feel like leaving the room, actually",
        None,
    );

    items.push(door);

    loop {
        clear_background(DARKGRAY);

        for item in &items {
            draw_texture(item.texture, item.position.x, item.position.y, WHITE);
        }

        next_frame().await
    }
}
