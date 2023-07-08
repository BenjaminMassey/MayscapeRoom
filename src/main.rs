use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
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
        Pos {
            x: tuple.0,
            y: tuple.1,
        }
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
enum Room {
    North,
    South,
    East,
    West,
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
        Bounds {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
}

struct Item {
    room: Room,
    tag: String,
    texture: Texture2D,
    position: Pos,
    state: ItemState,
    flavor_text: Vec<String>,
    link: Option<Box<Item>>,
}

impl Item {
    fn new(
        room: Room,
        tag: &str,
        texture: Texture2D,
        position: Pos,
        state: ItemState,
        flavor_text: Vec<&str>,
        link: Option<Box<Item>>,
    ) -> Self {
        Item {
            room,
            tag: tag.to_owned(),
            texture,
            position,
            state,
            flavor_text: flavor_text.into_iter().map(|a| a.to_owned()).collect(),
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
        point.x > bounds.top_left.x
            && point.x < bounds.top_right.x
            && point.y > bounds.top_right.y
            && point.y < bounds.bottom_right.y
    }
}

fn rotate_left(current: Room) -> Room {
    match current {
        Room::North => Room::East,
        Room::East => Room::South,
        Room::South => Room::West,
        Room::West => Room::North,
    }
}

fn rotate_right(current: Room) -> Room {
    match current {
        Room::North => Room::West,
        Room::West => Room::South,
        Room::South => Room::East,
        Room::East => Room::North,
    }
}

#[macroquad::main("EscapeRoom")]
async fn main() {
    
    // "Globals" of sorts

    let mut main_text: Vec<String> = Vec::new();

    let mut items: Vec<Item> = Vec::new();

    let mut current_room: Room = Room::North;

    // UI elements

    let left_arrow: Texture2D = load_texture("assets/ArrowLeft.png").await.unwrap();

    let right_arrow: Texture2D = load_texture("assets/ArrowRight.png").await.unwrap();

    // North room items

    let door_texture: Texture2D = load_texture("assets/ExitDoor.png").await.unwrap();
    let door = Item::new(
        Room::North,
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
        Room::North,
        "north_table",
        table_texture,
        Pos::new(10f32, 300f32),
        ItemState::Flavor,
        vec!["It's just a table, I think."],
        None,
    );
    items.push(table);

    // East Room Items

    let phonebooth_texture: Texture2D = load_texture("assets/PhoneBooth.png").await.unwrap();
    let phonebooth = Item::new(
        Room::East,
        "phonebooth",
        phonebooth_texture,
        Pos::new(40f32, 150f32),
        ItemState::Flavor,
        vec!["That thar be a phonebooth."],
        None,
    );
    items.push(phonebooth);

    loop {

        // Background

        let bg = match current_room {
            Room::North => Color::new(103f32 / 255f32, 118f32 / 255f32, 143f32 / 255f32, 1f32),
            Room::East => Color::new(96f32 / 255f32, 105f32 / 255f32, 120f32 / 255f32, 1f32),
            Room::South => Color::new(63f32 / 255f32, 72f32 / 255f32, 87f32 / 255f32, 1f32),
            Room::West => Color::new(72f32 / 255f32, 86f32 / 255f32, 110f32 / 255f32, 1f32),
        };
        clear_background(bg);

        // Store mouse pos, if clicked

        let mut mouse: Option<Pos> = None;
        if is_mouse_button_pressed(MouseButton::Left) {
            mouse = Some(Pos::tuple(mouse_position()));
        }

        // Main items loop, for drawing and clicking

        for item in &items {
            if item.room != current_room {
                continue;
            }

            draw_texture(item.texture, item.position.x, item.position.y, WHITE);

            if mouse.is_some() {
                if item.contains(mouse.unwrap()) {
                    if item.state == ItemState::Flavor {
                        main_text = item.flavor_text.clone();
                    }
                }
            }
        }

        // UI room-change arrows

        draw_texture(left_arrow, 0.0, 100.0, WHITE);

        draw_texture(right_arrow, 500.0, 100.0, WHITE);

        if mouse.is_some() {
            let m = mouse.unwrap();
            if m.x > 0.0 && m.x < 100.0 && m.y > 100.0 && m.y < 200.0 {
                current_room = rotate_left(current_room);
            }
            if m.x > 500.0 && m.x < 650.0 && m.y > 100.0 && m.y < 200.0 {
                current_room = rotate_right(current_room);
            }
        }

        // Draw any global text (flavor text from items)

        for (i, text) in main_text.iter().enumerate() {
            draw_text(&text, 20.0, 25.0 + ((i as f32) * 25.0), 30.0, WHITE);
        }

        // Show which room in top right

        let direction = match current_room {
            Room::North => "N",
            Room::East => "E",
            Room::South => "S",
            Room::West => "W",
        };
        draw_text(&direction, 605.0, 40.0, 50.0, RED);

        next_frame().await
    }
}
