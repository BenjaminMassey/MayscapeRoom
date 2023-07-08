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

#[derive(PartialEq, Clone)]
enum Room {
    None,
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum UserState {
    Nothing,
    Looking,
    Interacting,
}

#[derive(PartialEq, Clone)]
enum ItemState {
    Nothing,
    Flavor,
    Look,
    Interact,
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

#[derive(PartialEq, Clone)]
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
        Room::None => Room::None,
    }
}

fn rotate_right(current: Room) -> Room {
    match current {
        Room::North => Room::West,
        Room::West => Room::South,
        Room::South => Room::East,
        Room::East => Room::North,
        Room::None => Room::None,
    }
}

#[macroquad::main("EscapeRoom")]
async fn main() {

    // "Globals" of sorts

    let mut main_text: Vec<String> = Vec::new();

    let mut items: Vec<Item> = Vec::new();

    let mut current_room: Room = Room::North;

    let mut current_state: UserState = UserState::Nothing;

    let mut current_item: Option<Item> = None;

    let mut door_pad_entry: Vec<i16> = vec![1, 1, 1, 1];

    // UI elements

    let left_arrow: Texture2D = load_texture("assets/ArrowLeft.png").await.unwrap();

    let right_arrow: Texture2D = load_texture("assets/ArrowRight.png").await.unwrap();

    // North room items

    let door_pad_texture: Texture2D = load_texture("assets/ExitDoorPad.png").await.unwrap();
    let door_pad = Item::new(
        Room::None,
        "door_pad",
        door_pad_texture,
        Pos::new(125f32, 25f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(door_pad.clone());

    let door_texture: Texture2D = load_texture("assets/ExitDoor.png").await.unwrap();
    let door = Item::new(
        Room::North,
        "exit_door",
        door_texture,
        Pos::new(100f32, 0f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(door_pad.clone())),
    );
    items.push(door);

    let table_texture: Texture2D = load_texture("assets/Table.png").await.unwrap();
    let table = Item::new(
        Room::North,
        "north_table",
        table_texture,
        Pos::new(10f32, 300f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(table);

    let north_book_texture: Texture2D = load_texture("assets/NorthBook.png").await.unwrap();
    let north_open_book = Item::new(
        Room::None,
        "north_open_book",
        north_book_texture,
        Pos::new(50f32, 50f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(north_open_book.clone());

    let closed_book_texture: Texture2D = load_texture("assets/Book.png").await.unwrap();
    let north_closed_book = Item::new(
        Room::North,
        "north_closed_book",
        closed_book_texture,
        Pos::new(50f32, 335f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(north_open_book.clone())),
    );
    items.push(north_closed_book);

    let north_big_painting_texture: Texture2D = load_texture("assets/NorthPaintingBig.png").await.unwrap();
    let north_big_painting = Item::new(
        Room::None,
        "north_big_painting",
        north_big_painting_texture,
        Pos::new(200f32, 0f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(north_big_painting.clone());

    let north_small_painting_texture: Texture2D = load_texture("assets/NorthPaintingSmall.png").await.unwrap();
    let north_small_painting = Item::new(
        Room::North,
        "north_small_painting",
        north_small_painting_texture,
        Pos::new(460f32, 225f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(north_big_painting.clone())),
    );
    items.push(north_small_painting);

    let clock_big_texture: Texture2D = load_texture("assets/ClockBig.png").await.unwrap();
    let big_clock = Item::new(
        Room::None,
        "big_clock",
        clock_big_texture,
        Pos::new(100f32, 0f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(big_clock.clone());

    let clock_small_texture: Texture2D = load_texture("assets/ClockSmall.png").await.unwrap();
    let small_clock = Item::new(
        Room::North,
        "small_clock",
        clock_small_texture,
        Pos::new(420f32, 25f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(big_clock.clone())),
    );
    items.push(small_clock);

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

        // Background by room

        let bg = match current_room {
            Room::North => Color::new(103f32 / 255f32, 118f32 / 255f32, 143f32 / 255f32, 1f32),
            Room::East => Color::new(96f32 / 255f32, 105f32 / 255f32, 120f32 / 255f32, 1f32),
            Room::South => Color::new(63f32 / 255f32, 72f32 / 255f32, 87f32 / 255f32, 1f32),
            Room::West => Color::new(72f32 / 255f32, 86f32 / 255f32, 110f32 / 255f32, 1f32),
            Room::None => WHITE,
        };
        clear_background(bg);

        // Store mouse pos, if clicked

        let mut mouse: Option<Pos> = None;
        if is_mouse_button_pressed(MouseButton::Left) {
            mouse = Some(Pos::tuple(mouse_position()));
        }

        // Handle default state of looking around the room

        if current_state == UserState::Nothing { 

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
                            current_state = UserState::Nothing;
                            current_item = None;
                        }
                        else if item.state == ItemState::Look {
                            current_state = UserState::Looking;
                            current_item = Some(item.clone());
                            main_text = vec!["".to_string()];
                        }
                        else if item.state == ItemState::Interact {
                            current_state = UserState::Interacting;
                            current_item = Some(item.clone());
                            main_text = vec!["".to_string()];
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
                    main_text = vec!["".to_string()];
                }
                if m.x > 500.0 && m.x < 650.0 && m.y > 100.0 && m.y < 200.0 {
                    current_room = rotate_right(current_room);
                    main_text = vec!["".to_string()];
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
                Room::None => "Err",
            };
            draw_text(&direction, 605.0, 40.0, 50.0, RED);

        }

        // Handle state of currently looking at an item, should be some guarantees

        else if current_state == UserState::Looking {

            // Show linked item

            let item = current_item.clone().unwrap().link.unwrap().clone();
            draw_texture(
                item.texture,
                item.position.x,
                item.position.y,
                WHITE
            );

            // Give UI go back button

            draw_texture(left_arrow, 0.0, 20.0, WHITE);

            if mouse.is_some() {
                let m = mouse.unwrap();
                if m.x > 0.0 && m.x < 100.0 && m.y > 20.0 && m.y < 120.0 {
                    current_state = UserState::Nothing;
                    current_item = None;
                }
            }
        }

        // Handle state of interacting with object, going to be specific to item

        else if current_state == UserState::Interacting {

            // Do main texture drawing

            let item = current_item.clone().unwrap().link.unwrap().clone();
            draw_texture(
                item.texture,
                item.position.x,
                item.position.y,
                WHITE
            );

            // Handle specific states by item

            if item.tag == "door_pad" {
                draw_text(&door_pad_entry[0].to_string(), 200.0, 240.0, 80.0, BLACK);
                draw_text(&door_pad_entry[1].to_string(), 285.0, 245.0, 80.0, BLACK);
                draw_text(&door_pad_entry[2].to_string(), 370.0, 240.0, 80.0, BLACK);
                draw_text(&door_pad_entry[3].to_string(), 445.0, 233.0, 80.0, BLACK);
                if mouse.is_some() {
                    let m = mouse.unwrap();
                    if m.x > 200.0 && m.x < 250.0 && m.y > 150.0 && m.y < 285.0 {
                        door_pad_entry[0] = (door_pad_entry[0] + 1) % 10
                    } else if m.x > 285.0 && m.x < 335.0 && m.y > 150.0 && m.y < 285.0 {
                        door_pad_entry[1] = (door_pad_entry[1] + 1) % 10
                    } else if m.x > 370.0 && m.x < 420.0 && m.y > 150.0 && m.y < 285.0 {
                        door_pad_entry[2] = (door_pad_entry[2] + 1) % 10
                    } else if m.x > 445.0 && m.x < 495.0 && m.y > 150.0 && m.y < 285.0 {
                        door_pad_entry[3] = (door_pad_entry[3] + 1) % 10
                    } else if m.x > 265.0 && m.x < 392.0 && m.y > 345.0 && m.y < 390.0 {
                        // Confirm button pressed
                        if door_pad_entry == vec![1, 2, 3, 4] {

                            // TODO: open door essentially goes over the original door
                            //       successfully, but I'd still rather remove that
                            //       original door: below doesn't work
                            //items.retain(|x| x.clone() != door.clone());

                            let open_door_texture: Texture2D = load_texture("assets/OpenDoor.png").await.unwrap();
                            let open_door = Item::new(
                                Room::North,
                                "open_door",
                                open_door_texture,
                                Pos::new(100f32, 0f32),
                                ItemState::Flavor,
                                vec!["You know, I actually don't really", "feel like leaving, actually."],
                                None,
                            );
                            items.push(open_door);

                            main_text = vec!["The door opened!".to_string()];
                            current_state = UserState::Nothing;
                            current_item = None;
                        }
                        else {
                            println!("WRONG"); // TODO: error sound
                        }
                    }
                }
            }

            // Give UI to go back

            draw_texture(left_arrow, 0.0, 20.0, WHITE);

            if mouse.is_some() {
                let m = mouse.unwrap();
                if m.x > 0.0 && m.x < 100.0 && m.y > 20.0 && m.y < 120.0 {
                    current_state = UserState::Nothing;
                    current_item = None;
                }
            }
        }

        next_frame().await
    }
}
