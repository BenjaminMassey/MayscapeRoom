use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use macroquad::input::{is_mouse_button_pressed, MouseButton, mouse_position};
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn new(x: f32, y: f32) -> Self {
        Pos { x, y }
    }
    fn tuple(tuple: (f32, f32)) -> Self {
        Pos { x: tuple.0, y: tuple.1 }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq)]
enum ItemState {
    Nothing,
    Flavor,
    Linked,
}

struct Bounds {
    top_left: Pos,
    top_right: Pos,
    bottom_left: Pos,
    bottom_right: Pos,
}

impl Bounds {
    fn new(top_left: Pos, top_right: Pos, bottom_left: Pos, bottom_right: Pos) -> Self {
        Bounds { top_left, top_right, bottom_left, bottom_right }
    }
}

struct Item {
    tag: String,
    texture: Texture2D,
    position: Pos,
    state: ItemState,
    flavor_text: Vec<String>,
    link: Option<Box<Item>>,
}

impl Item {
    fn new(
        tag: &str,
        texture: Texture2D,
        position: Pos,
        state: ItemState,
        flavor_text: Vec<&str>,
        link: Option<Box<Item>>,
    ) -> Self {
        Item {
            tag: tag.to_owned(),
            texture,
            position,
            state,
            flavor_text:
                flavor_text
                .into_iter()
                .map(|a| { a.to_owned() })
                .collect(),
            link,
        }
    }
    fn bounds(&self) -> Bounds {
        Bounds::new(
            self.position,
            self.position + Pos::new(self.texture.width(), 0f32),
            self.position + Pos::new(0f32, self.texture.height()),
            self.position + Pos::new(self.texture.width(), self.texture.height()),
        )
    }
    fn contains(&self, point: Pos) -> bool {
        let bounds = self.bounds();
        point.x > bounds.top_left.x &&
            point.x < bounds.top_right.x &&
            point.y > bounds.top_right.y &&
            point.y < bounds.bottom_right.y
    }
}

#[macroquad::main("EscapeRoom")]
async fn main() {
    let mut main_text: Vec<String> = Vec::new();

    let mut items: Vec<Item> = Vec::new();

    let door_texture: Texture2D = load_texture("assets/ExitDoor.png").await.unwrap();
    let door = Item::new(
        "exit_door",
        door_texture,
        Pos::new(100f32, 0f32),
        ItemState::Flavor,
        vec!["I don't really feel like", "leaving the room, actually."],
        None, // TODO
    );
    items.push(door);

    let table_texture: Texture2D = load_texture("assets/Table.png").await.unwrap();
    let table = Item::new(
        "north_table",
        table_texture,
        Pos::new(10f32, 300f32),
        ItemState::Flavor,
        vec!["It's just a table, I think."],
        None,
    );
    items.push(table);

    loop {
        clear_background(DARKGRAY);

        let mut mouse: Option<Pos> = None;

        if is_mouse_button_pressed(MouseButton::Left) {
            mouse = Some(Pos::tuple(mouse_position()));
        }

        for item in &items{
            draw_texture(item.texture, item.position.x, item.position.y, WHITE);
            if mouse.is_some() {
                if item.contains(mouse.unwrap()) {
                    if item.state == ItemState::Flavor {
                        main_text = item.flavor_text.clone();
                    }
                }
            }
        }

        for (i, text) in main_text.iter().enumerate() {
            draw_text(&text, 20.0, 25.0 + ((i as f32) * 25.0), 30.0, WHITE);
        }

        next_frame().await
    }
}
