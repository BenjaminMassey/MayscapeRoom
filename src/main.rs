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
    Complete,
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

    let mut color_match_wires: Vec<Option<i16>> = vec![None, None, None, None];

    let mut current_wire: Option<&str> = None;

    let mut phone_number: String = "".to_string();

    let mut safe_entry: Vec<i16> = vec![1, 1, 1, 1];

    let mut candle_placement: Vec<i16> = vec![3, 2, 1, 0];

    let mut code_entry: Vec<i16> = vec![0, 0, 0, 0];

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
    let north_table = Item::new(
        Room::North,
        "north_table",
        table_texture,
        Pos::new(10f32, 300f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(north_table);

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

    let phone_entry_texture: Texture2D = load_texture("assets/PhoneEntry.png").await.unwrap();
    let phone_entry = Item::new(
        Room::None,
        "phone_entry",
        phone_entry_texture,
        Pos::new(180f32, 0f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(phone_entry.clone());

    let phonebooth_texture: Texture2D = load_texture("assets/PhoneBooth.png").await.unwrap();
    let phonebooth = Item::new(
        Room::East,
        "phonebooth",
        phonebooth_texture,
        Pos::new(100f32, 50f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(phone_entry.clone())),
    );
    items.push(phonebooth);

    let shelf_texture: Texture2D = load_texture("assets/Shelf.png").await.unwrap();
    let east_shelf = Item::new(
        Room::East,
        "east_shelf",
        shelf_texture,
        Pos::new(125f32, 350f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(east_shelf);

    let east_book_texture: Texture2D = load_texture("assets/EastBook.png").await.unwrap();
    let east_book = Item::new(
        Room::None,
        "east_book",
        east_book_texture,
        Pos::new(150f32, 75f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(east_book.clone());

    let east_closed_book = Item::new(
        Room::East,
        "east_closed_book",
        closed_book_texture,
        Pos::new(175f32, 300f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(east_book.clone())),
    );
    items.push(east_closed_book);

    let east_big_painting_texture: Texture2D = load_texture("assets/WashingtonBig.png").await.unwrap();
    let east_big_painting = Item::new(
        Room::None,
        "east_big_painting",
        east_big_painting_texture,
        Pos::new(180f32, 30f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(east_big_painting.clone());

    let east_small_painting_texture: Texture2D = load_texture("assets/WashingtonSmall.png").await.unwrap();
    let east_small_painting = Item::new(
        Room::East,
        "east_small_painting",
        east_small_painting_texture,
        Pos::new(360f32, 175f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(east_big_painting.clone())),
    );
    items.push(east_small_painting);

    let east_table = Item::new(
        Room::East,
        "north_table",
        table_texture,
        Pos::new(410f32, 300f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(east_table);

    let colormatch_texture: Texture2D = load_texture("assets/ColorMatch.png").await.unwrap();
    let colormatch = Item::new(
        Room::None,
        "colormatch",
        colormatch_texture,
        Pos::new(180f32, 5f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(colormatch.clone());

    let colorbox_texture: Texture2D = load_texture("assets/ColorBox.png").await.unwrap();
    let colorbox = Item::new(
        Room::East,
        "colorbox",
        colorbox_texture,
        Pos::new(460f32, 350f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(colormatch).clone()),
    );
    items.push(colorbox);

    // West room items

    let weights_big_texture: Texture2D = load_texture("assets/WeightsBig.png").await.unwrap();
    let weight_big = Item::new(
        Room::None,
        "weight_big",
        weights_big_texture,
        Pos::new(100f32, 5f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(weight_big.clone());

    let weights_small_texture: Texture2D = load_texture("assets/WeightsSmall.png").await.unwrap();
    let weights_small = Item::new(
        Room::West,
        "weights_small",
        weights_small_texture,
        Pos::new(50f32, 300f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(weight_big).clone()),
    );
    items.push(weights_small);

    let west_table = Item::new(
        Room::West,
        "west_table",
        table_texture,
        Pos::new(410f32, 300f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(west_table);

    let paint_numbers_big_texture: Texture2D = load_texture("assets/PaintNumbersBig.png").await.unwrap();
    let paint_numbers_big = Item::new(
        Room::None,
        "paint_numbers_big",
        paint_numbers_big_texture,
        Pos::new(100f32, 5f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(paint_numbers_big.clone());

    let paint_numbers_small_texture: Texture2D = load_texture("assets/PaintNumbersSmall.png").await.unwrap();
    let paint_numbers_small = Item::new(
        Room::West,
        "paint_numbers_small",
        paint_numbers_small_texture,
        Pos::new(460f32, 325f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(paint_numbers_big).clone()),
    );
    items.push(paint_numbers_small);

    let west_shelf = Item::new(
        Room::West,
        "west_shelf",
        shelf_texture,
        Pos::new(325f32, 150f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(west_shelf);

    let window_texture: Texture2D = load_texture("assets/Window.png").await.unwrap();
    let window = Item::new(
        Room::West,
        "window",
        window_texture,
        Pos::new(150f32, 50f32),
        ItemState::Flavor,
        vec!["What a nice view!"],
        None,
    );
    items.push(window);

    let safe_big_texture: Texture2D = load_texture("assets/SafeBig.png").await.unwrap();
    let safe_big = Item::new(
        Room::None,
        "safe_big",
        safe_big_texture,
        Pos::new(100f32, 5f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(safe_big.clone());

    let safe_small_texture: Texture2D = load_texture("assets/SafeSmall.png").await.unwrap();
    let safe_small = Item::new(
        Room::West,
        "safe_small",
        safe_small_texture,
        Pos::new(390f32, 95f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(safe_big).clone()),
    );
    items.push(safe_small);

    // South room items

    let light_texture: Texture2D = load_texture("assets/Light.png").await.unwrap();
    let light = Item::new(
        Room::South,
        "light",
        light_texture,
        Pos::new(100f32, 0f32),
        ItemState::Flavor,
        vec!["An ugly but functional light fixture.", "It came with the place."],
        None,
    );
    items.push(light);

    let vase_big_texture: Texture2D = load_texture("assets/VaseBig.png").await.unwrap();
    let vase_big = Item::new(
        Room::None,
        "vase_big",
        vase_big_texture,
        Pos::new(100f32, 0f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(vase_big.clone());

    let vase_small_texture: Texture2D = load_texture("assets/VaseSmall.png").await.unwrap();
    let vase_small = Item::new(
        Room::South,
        "vase_small",
        vase_small_texture,
        Pos::new(140f32, 310f32),
        ItemState::Look,
        vec![""],
        Some(Box::new(vase_big).clone()),
    );
    items.push(vase_small);

    let south_table = Item::new(
        Room::South,
        "south_table",
        table_texture,
        Pos::new(100f32, 300f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(south_table);

    let candlecase_big_texture: Texture2D = load_texture("assets/CandleCaseBig.png").await.unwrap();
    let candlecase_big = Item::new(
        Room::None,
        "candlecase_big",
        candlecase_big_texture,
        Pos::new(100f32, 5f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(candlecase_big.clone());

    let candlecase_small_texture: Texture2D = load_texture("assets/CandleCaseSmall.png").await.unwrap();
    let candlecase_small = Item::new(
        Room::South,
        "candlecase_small",
        candlecase_small_texture,
        Pos::new(340f32, 160f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(candlecase_big).clone()),
    );
    items.push(candlecase_small);

    let candle_a: Texture2D = load_texture("assets/CandleA.png").await.unwrap();
    let candle_b: Texture2D = load_texture("assets/CandleB.png").await.unwrap();
    let candle_c: Texture2D = load_texture("assets/CandleC.png").await.unwrap();
    let candle_d: Texture2D = load_texture("assets/CandleD.png").await.unwrap();

    let codeentry_big_texture: Texture2D = load_texture("assets/CodeEntryBig.png").await.unwrap();
    let codeentry_big = Item::new(
        Room::None,
        "codeentry_big",
        codeentry_big_texture,
        Pos::new(25f32, 50f32),
        ItemState::Nothing,
        vec![""],
        None,
    );
    items.push(codeentry_big.clone());

    let codeentry_small_texture: Texture2D = load_texture("assets/CodeEntrySmall.png").await.unwrap();
    let codeentry_small = Item::new(
        Room::South,
        "codeentry_small",
        codeentry_small_texture,
        Pos::new(400f32, 325f32),
        ItemState::Interact,
        vec![""],
        Some(Box::new(codeentry_big).clone()),
    );
    items.push(codeentry_small);

    let code_apple: Texture2D = load_texture("assets/CodeApple.png").await.unwrap();
    let code_beaver: Texture2D = load_texture("assets/CodeBeaver.png").await.unwrap();
    let code_cat: Texture2D = load_texture("assets/CodeCat.png").await.unwrap();
    let code_cactus: Texture2D = load_texture("assets/CodeCactus.png").await.unwrap();
    let code_dog: Texture2D = load_texture("assets/CodeDog.png").await.unwrap();
    let code_grass: Texture2D = load_texture("assets/CodeGrass.png").await.unwrap();
    let code_log: Texture2D = load_texture("assets/CodeLog.png").await.unwrap();
    let code_man: Texture2D = load_texture("assets/CodeMan.png").await.unwrap();
    let code_orange: Texture2D = load_texture("assets/CodeOrange.png").await.unwrap();
    let code_pumpkin: Texture2D = load_texture("assets/CodePumpkin.png").await.unwrap();
    let code_raspberry: Texture2D = load_texture("assets/CodeRaspberry.png").await.unwrap();
    let code_snail: Texture2D = load_texture("assets/CodeSnail.png").await.unwrap();
    let code_sunflower: Texture2D = load_texture("assets/CodeSunflower.png").await.unwrap();

    let code_textures: Vec<Texture2D> = 
        vec![
            code_grass,
            code_beaver,
            code_cat, // correct (2)
            code_dog,
            code_apple, // correct (4)
            code_log,
            code_man,
            code_cactus, // correct (7)
            code_orange,
            code_pumpkin,
            code_raspberry, // correct (10)
            code_snail,
            code_sunflower,
        ]
    ;

    loop {

        // Background by room

        let bg = match current_room { // TODO: prob shouldn't generate colors every frame
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
                                vec!["You know, I don't really", "feel like leaving, actually."],
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

            else if item.tag == "colormatch" {

                // Render lines

                let red_left = Pos::new(278.0, 65.0);
                let red_right = Pos::new(480.0, 65.0);
                let green_left = Pos::new(294.0, 151.0);
                let green_right = Pos::new(479.0, 148.0);
                let blue_left = Pos::new(299.0, 231.0);
                let blue_right = Pos::new(480.0, 227.0);
                let orange_left = Pos::new(309.0, 323.0);
                let orange_right = Pos::new(478.0, 327.0);
                let rights: Vec<Pos> = vec![
                    red_right,
                    green_right,
                    blue_right,
                    orange_right,
                ];
                if color_match_wires[0].is_some() {
                    let right = rights[color_match_wires[0].unwrap() as usize];
                    draw_line(red_left.x, red_left.y, right.x, right.y, 15.0, GRAY);
                }
                if color_match_wires[1].is_some() {
                    let right = rights[color_match_wires[1].unwrap() as usize];
                    draw_line(green_left.x, green_left.y, right.x, right.y, 15.0, GRAY);
                }
                if color_match_wires[2].is_some() {
                    let right = rights[color_match_wires[2].unwrap() as usize];
                    draw_line(blue_left.x, blue_left.y, right.x, right.y, 15.0, GRAY);
                }
                if color_match_wires[3].is_some() {
                    let right = rights[color_match_wires[3].unwrap() as usize];
                    draw_line(orange_left.x, orange_left.y, right.x, right.y, 15.0, GRAY);
                }

                // Handle input

                let mut spot_tap: Option<&str> = None;

                if mouse.is_some() {
                    let m = mouse.unwrap();
                    if m.x > red_left.x - 65.0 && m.x < red_left.x
                        && m.y > red_left.y - 20.0 && m.y < red_left.y + 20.0 {
                        spot_tap = Some("red_left");
                    } else if m.x > green_left.x - 65.0 && m.x < green_left.x
                        && m.y > green_left.y - 20.0 && m.y < green_left.y + 20.0 {
                        spot_tap = Some("green_left");
                    } else if m.x > blue_left.x - 65.0 && m.x < blue_left.x
                        && m.y > blue_left.y - 20.0 && m.y < blue_left.y + 20.0 {
                        spot_tap = Some("blue_left");
                    } else if m.x > orange_left.x - 65.0 && m.x < orange_left.x
                        && m.y > orange_left.y - 20.0 && m.y < orange_left.y + 20.0 {
                        spot_tap = Some("orange_left");
                    } else if m.x > red_right.x && m.x < red_right.x + 65.0
                        && m.y > red_right.y - 20.0 && m.y < red_right.y + 20.0 {
                        spot_tap = Some("red_right");
                    } else if m.x > green_right.x && m.x < green_right.x + 65.0
                        && m.y > green_right.y - 20.0 && m.y < green_right.y + 20.0 {
                        spot_tap = Some("green_right");
                    } else if m.x > blue_right.x && m.x < blue_right.x + 65.0
                        && m.y > blue_right.y - 20.0 && m.y < blue_right.y + 20.0 {
                        spot_tap = Some("blue_right");
                    } else if m.x > orange_right.x && m.x < orange_right.x + 65.0
                        && m.y > orange_right.y - 20.0 && m.y < orange_right.y + 20.0 {
                        spot_tap = Some("orange_right");
                    }
                }

                if spot_tap.is_some() {
                    let new = spot_tap.unwrap();
                    if current_wire.is_some() {
                        let last = current_wire.unwrap();

                        let last_parts = last.split("_").collect::<Vec<&str>>();
                        let new_parts = new.split("_").collect::<Vec<&str>>();

                        let last_is_left = last_parts[1] == "left";
                        let new_is_left = new_parts[1] == "left";

                        let mut left_string: &str = "";
                        let mut right_string: &str = "";

                        if new_is_left && !last_is_left {
                            left_string = new_parts[0];
                            right_string = last_parts[0];
                        } else if !new_is_left && last_is_left {
                            left_string = last_parts[0];
                            right_string = new_parts[0];
                        }
                        if left_string.len() > 0 && right_string.len() > 0 {
                            let index = match left_string {
                                "red" => 0,
                                "green" => 1,
                                "blue" => 2,
                                "orange" => 3,
                                _ => 0,
                            };
                            let value = match right_string {
                                "red" => 0,
                                "green" => 1,
                                "blue" => 2,
                                "orange" => 3,
                                _ => 0,
                            };
                            color_match_wires[index] = Some(value);
                            current_wire = None;
                        }
                    }
                    else {
                        current_wire = Some(new);
                    }
                }

                // Give result text at the bottom

                let mut result_text: &str = "Err";

                if color_match_wires == vec![Some(0), Some(1), Some(2), Some(3)] {
                    result_text = "1234";
                } else if color_match_wires == vec![Some(1), Some(0), Some(3), Some(2)] {
                    result_text = "1776!";
                }

                draw_text(result_text, 350.0, 450.0, 50.0, WHITE);
            }

            else if item.tag == "phone_entry" {
                if mouse.is_some() {
                    let m = mouse.unwrap();
                    let mut hit: Option<&str> = None;
                    if m.x > 274.0 && m.x < 327.0 && m.y > 128.0 && m.y < 170.0 {
                        hit = Some("0");
                    } else if m.x > 217.0 && m.x < 267.0 && m.y > 197.0 && m.y < 247.0 {
                        hit = Some("1");
                    } else if m.x > 284.0 && m.x < 330.0 && m.y > 189.0 && m.y < 241.0 {
                        hit = Some("2");
                    } else if m.x > 357.0 && m.x < 398.0 && m.y > 189.0 && m.y < 245.0 {
                        hit = Some("3");
                    } else if m.x > 221.0 && m.x < 265.0 && m.y > 268.0 && m.y < 312.0 {
                        hit = Some("4");
                    } else if m.x > 291.0 && m.x < 333.0 && m.y > 267.0 && m.y < 307.0 {
                        hit = Some("5");
                    } else if m.x > 363.0 && m.x < 405.0 && m.y > 267.0 && m.y < 307.0 {
                        hit = Some("6");
                    } else if m.x > 227.0 && m.x < 272.0 && m.y > 333.0 && m.y < 372.0 {
                        hit = Some("7");
                    } else if m.x > 296.0 && m.x < 338.0 && m.y > 328.0 && m.y < 369.0 {
                        hit = Some("8");
                    } else if m.x > 367.0 && m.x < 411.0 && m.y > 331.0 && m.y < 365.0 {
                        hit = Some("9");
                    }
                    if hit.is_some() {
                        let num = hit.unwrap();
                        if phone_number == "pumpkin"
                            || phone_number == "raspberry"
                            || phone_number == "sunflower"
                            || phone_number == "cactus"
                            || phone_number == "INCORRECT" {
                            phone_number = "".to_string();
                        }
                        phone_number = phone_number + num;
                    }
                    else {
                        if m.x > 460.0 && m.x < 554.0 && m.y > 183.0 && m.y < 244.0 {
                            // TOOD: real sounds
                            if phone_number == "1234" {
                                phone_number = "pumpkin".to_string();
                            } else if phone_number == "8659" {
                                phone_number = "raspberry".to_string();
                            } else if phone_number == "1776" {
                                phone_number = "sunflower".to_string();
                            } else if phone_number == "150405040720" {
                                phone_number = "cactus".to_string();
                            } else {
                                phone_number = "INCORRECT".to_string();
                            }
                        } else if m.x > 460.0 && m.x < 557.0 && m.y > 263.0 && m.y < 332.0 {
                            phone_number = "".to_string();
                        }
                    }
                }

                draw_text(&phone_number, 260.0, 435.0, 50.0, WHITE);
            }

            else if item.tag == "safe_big" {
                draw_text(&safe_entry[0].to_string(), 200.0, 120.0, 80.0, BLACK);
                draw_text(&safe_entry[1].to_string(), 270.0, 120.0, 80.0, BLACK);
                draw_text(&safe_entry[2].to_string(), 340.0, 120.0, 80.0, BLACK);
                draw_text(&safe_entry[3].to_string(), 410.0, 120.0, 80.0, BLACK);
                if mouse.is_some() {
                    let m = mouse.unwrap();
                    println!("{}, {}", m.x, m.y);
                    if m.x > 200.0 && m.x < 250.0 && m.y > 90.0 && m.y < 150.0 {
                        safe_entry[0] = (safe_entry[0] + 1) % 10
                    } else if m.x > 270.0 && m.x < 330.0 && m.y > 90.0 && m.y < 150.0 {
                        safe_entry[1] = (safe_entry[1] + 1) % 10
                    } else if m.x > 340.0 && m.x < 390.0 && m.y > 90.0 && m.y < 150.0 {
                        safe_entry[2] = (safe_entry[2] + 1) % 10
                    } else if m.x > 410.0 && m.x < 460.0 && m.y > 90.0 && m.y < 150.0 {
                        safe_entry[3] = (safe_entry[3] + 1) % 10
                    } else if m.x > 362.0 && m.x < 474.0 && m.y > 188.0 && m.y < 298.0 {
                        // Confirm button pressed
                        if safe_entry == vec![5, 3, 9, 4] {

                            // TODO: open safe essentially goes over the original safe
                            //       successfully, but I'd still rather remove that
                            //       original safe: below doesn't work
                            //items.retain(|x| x.clone() != safe_small.clone());

                            let safe_big_texture: Texture2D = load_texture("assets/OpenSafeBig.png").await.unwrap();
                            let safe_big = Item::new(
                                Room::None,
                                "safe_big",
                                safe_big_texture,
                                Pos::new(100f32, 0f32),
                                ItemState::Nothing,
                                vec![""],
                                None,
                            );
                            items.push(safe_big.clone());

                            let safe_small_texture: Texture2D = load_texture("assets/OpenSafeSmall.png").await.unwrap();
                            let safe_small = Item::new(
                                Room::West,
                                "safe_small",
                                safe_small_texture,
                                Pos::new(390f32, 95f32),
                                ItemState::Look,
                                vec![""],
                                Some(Box::new(safe_big.clone())),
                            );
                            items.push(safe_small);

                            main_text = vec!["The safe opened!".to_string()];
                            current_state = UserState::Nothing;
                            current_item = None;
                        }
                        else {
                            println!("WRONG"); // TODO: error sound
                        }
                    }
                }
            }

            else if item.tag == "candlecase_big" {

                for i in 0..4 {
                    let candle = match candle_placement[i] {
                        0i16 => candle_a,
                        1i16 => candle_b,
                        2i16 => candle_c,
                        3i16 => candle_d,
                        _ => candle_a,
                    };

                    draw_texture(candle, 120.0 + (i as f32 * 120.0), 127.0, WHITE);
                }

                if mouse.is_some() {
                    let m = mouse.unwrap();
                    if m.x > 157.0 && m.x < 212.0 && m.y > 198.0 && m.y < 226.0 {
                        let zero_orig = candle_placement[0];
                        candle_placement[0] = candle_placement[1];
                        candle_placement[1] = zero_orig;
                    } else if m.x > 296.0 && m.x < 343.0 && m.y > 202.0 && m.y < 224.0 {
                        let one_orig = candle_placement[1];
                        candle_placement[1] = candle_placement[2];
                        candle_placement[2] = one_orig;
                    } else if m.x > 415.0 && m.x < 470.0 && m.y > 193.0 && m.y < 224.0 {
                        let two_orig = candle_placement[2];
                        candle_placement[2] = candle_placement[3];
                        candle_placement[3] = two_orig;
                    }
                }

                let mut answer = "Incorrect";

                if candle_placement == vec![2, 0, 3, 1] {
                    answer = "BEAVER";
                } else if candle_placement == vec![0, 1, 2, 3] {
                    answer = "CAT";
                }

                draw_text(&answer, 245.0, 400.0, 50.0, YELLOW);
            }

            else if item.tag == "codeentry_big" {
                draw_texture(code_textures[code_entry[0] as usize], 140.0, 230.0, WHITE);
                draw_texture(code_textures[code_entry[1] as usize], 240.0, 230.0, WHITE);
                draw_texture(code_textures[code_entry[2] as usize], 340.0, 230.0, WHITE);
                draw_texture(code_textures[code_entry[3] as usize], 440.0, 230.0, WHITE);
                if mouse.is_some() {
                    let m = mouse.unwrap();
                    if m.x > 140.0 && m.x < 190.0 && m.y > 180.0 && m.y < 280.0 {
                        code_entry[0] = (code_entry[0] + 1) % code_textures.len() as i16;
                    } else if m.x > 240.0 && m.x < 290.0 && m.y > 180.0 && m.y < 280.0 {
                        code_entry[1] = (code_entry[1] + 1) % code_textures.len() as i16;
                    } else if m.x > 340.0 && m.x < 390.0 && m.y > 180.0 && m.y < 280.0 {
                        code_entry[2] = (code_entry[2] + 1) % code_textures.len() as i16;
                    } else if m.x > 440.0 && m.x < 490.0 && m.y > 180.0 && m.y < 280.0 {
                        code_entry[3] = (code_entry[3] + 1) % code_textures.len() as i16;
                    } else if m.x > 224.0 && m.x < 423.0 && m.y > 386.0 && m.y < 468.0 {
                        // Confirm button pressed
                        if (code_entry[0] == 2 || code_entry[1] == 2 || code_entry[2] == 2 || code_entry[3] == 2)
                            &&  (code_entry[0] == 4 || code_entry[1] == 4 || code_entry[2] == 4 || code_entry[3] == 4) 
                            &&  (code_entry[0] == 7 || code_entry[1] == 7 || code_entry[2] == 7 || code_entry[3] == 7) 
                            &&  (code_entry[0] == 10 || code_entry[1] == 10 || code_entry[2] == 10 || code_entry[3] == 10) {
                            current_state = UserState::Complete;
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
                    current_wire = None;
                }
            }
        }

        // Handle the game being finished

        else if current_state == UserState::Complete {
            draw_text("After enough flailing around, you", 20.0, 100.0, 38.0, WHITE);
            draw_text("finally manage to solve the secret", 20.0, 140.0, 38.0, WHITE);
            draw_text("puzzle (ignoring an easy escape).", 20.0, 180.0, 38.0, WHITE);
            draw_text("You find yourself even more trapped", 20.0, 220.0, 38.0, WHITE);
            draw_text("within your room, unable to move.", 20.0, 260.0, 38.0, WHITE);
            draw_text("I suppose this was your goal?", 20.0, 315.0, 38.0, WHITE);
            draw_text("THE END", 200.0, 420.0, 80.0, YELLOW);
        }

        next_frame().await
    }
}
