use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::{collections::HashMap, usize};

use piston_window::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub colour: [f32; 4],
    pub tile_type: TileType,
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    Mountain,
    Land,
    Coast,
    Water,
}
impl Tile {
    pub fn new(tile_type: TileType, colour: [f32; 4]) -> Self {
        Tile {
            colour,
            tile_type,
            visible: true,
        }
    }

    pub fn empty() -> Self {
        Tile::new(TileType::Empty, [0.0, 0.0, 0.0, 0.0])
    }
    pub fn mountain() -> Self {
        Tile::new(TileType::Mountain, [0.5, 0.5, 0.5, 1.0])
    }
    pub fn land() -> Self {
        Tile::new(TileType::Land, [0.3, 0.8, 0.4, 1.0])
    }
    pub fn coast() -> Self {
        Tile::new(TileType::Coast, [0.8, 0.7, 0.6, 1.0])
    }
    pub fn water() -> Self {
        Tile::new(TileType::Water, [0.2, 0.4, 0.8, 1.0])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSystem {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: f64,
    pub grid_width: usize,
    pub grid_height: usize,
    pub window_width: f64,
    pub window_height: f64,
    pub saved_configs: HashMap<String, Vec<Vec<TileType>>>,
}

impl TileSystem {
    pub fn new(window_width: f64, window_height: f64, tile_size: f64) -> Self {
        let grid_width = (window_width / tile_size) as usize;
        let grid_height = (window_height / tile_size) as usize;

        let mut tiles = Vec::new();
        for _y in 0..grid_height {
            let mut row = Vec::new();
            for _x in 0..grid_width {
                row.push(Tile::empty());
            }
            tiles.push(row);
        }

        TileSystem {
            tiles,
            tile_size,
            grid_width,
            grid_height,
            window_width,
            window_height,
            saved_configs: HashMap::new(),
        }
    }

    // get tile at grid coords
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.grid_width && y < self.grid_height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) -> bool {
        if x < self.grid_width && y < self.grid_height {
            self.tiles[x][y] = tile;
            true
        } else {
            false
        }
    }

    pub fn save_config(&mut self, name: String) {
        let mut config = Vec::new();
        for row in &self.tiles {
            let mut config_row = Vec::new();
            for tile in row {
                config_row.push(tile.tile_type.clone());
            }
            config.push(config_row);
        }
        self.saved_configs.insert(name.clone(), config);
        println!("Saved configuration: {}", name);
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(filename, json_data)?;
        println!("Saved tile system to {}", filename);
        Ok(())
    }

    pub fn load_from_tile(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = fs::read_to_string(filename)?;
        let tile_system: TileSystem = serde_json::from_str(&json_data)?;
        println!("Loaded tile systems from {}", filename);
        Ok(tile_system)
    }

    pub fn load_config(&mut self, name: &str) -> bool {
        if let Some(config) = self.saved_configs.get(name) {
            for (y, row) in config.iter().enumerate() {
                for (x, tile_type) in row.iter().enumerate() {
                    if y < self.grid_height && x < self.grid_width {
                        let tile = match tile_type {
                            TileType::Empty => Tile::empty(),
                            TileType::Mountain => Tile::mountain(),
                            TileType::Land => Tile::land(),
                            TileType::Coast => Tile::coast(),
                            TileType::Water => Tile::water(),
                        };
                        self.tiles[y][x] = tile;
                    }
                }
            }
            println!("Loaded configuration: {}", name);
            true
        } else {
            println!("Configuration '{}' not found", name);
            false
        }
    }

    pub fn list_configs(&self) {
        if self.saved_configs.is_empty() {
            println!("No saved configurations");
        } else {
            println!("Saved configurations:");
            for name in self.saved_configs.keys() {
                println!(" - {}", name);
            }
        }
    }

    pub fn clear_map(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                *tile = Tile::empty();
            }
        }
        println!("Map cleared");
    }

    pub fn fill_to_border(&mut self, start_x: usize, start_y: usize, new_tile: Tile) {
        let original_tile = if let Some(tile) = self.get_tile(start_x, start_y) {
            tile.tile_type.clone()
        } else {
            return;
        };

        if original_tile == new_tile.tile_type {
            return;
        }

        let mut visited = vec![vec![false; self.grid_width]; self.grid_height];

        let mut stack = Vec::new();
        stack.push((start_x, start_y));

        while let Some((x, y)) = stack.pop() {
            if x >= self.grid_width || y >= self.grid_height {
                continue;
            }

            if visited[x][y] {
                continue;
            }

            if let Some(current_tile) = self.get_tile(x, y) {
                if current_tile.tile_type != original_tile {
                    continue;
                }
            } else {
                continue;
            }

            visited[x][y] = true;
            self.tiles[x][y] = new_tile.clone();

            //TODO: fix x and y flip flop thing.
            //left
            if x > 0 {
                stack.push((x - 1, y));
            }
            //right
            if x < self.grid_width - 1 {
                stack.push((x + 1, y));
            }
            //up
            if y > 0 {
                stack.push((x, y - 1));
            }
            //down
            if y < self.grid_height - 1 {
                stack.push((x, y + 1));
            }
        }
    }

    pub fn grid_to_world(&self, grid_x: usize, grid_y: usize) -> (f64, f64) {
        (
            grid_x as f64 * self.tile_size,
            grid_y as f64 * self.tile_size,
        )
    }

    pub fn get_tile_at_pos(&self, world_x: f64, world_y: f64) -> Option<(usize, usize)> {
        let grid_x = (world_x / self.tile_size) as usize;
        let grid_y = (world_y / self.tile_size) as usize;

        if grid_x < self.grid_width && grid_y < self.grid_height {
            Some((grid_x, grid_y))
        } else {
            None
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.visible && tile.colour[3] > 0.0 {
                    let (world_x, world_y) = self.grid_to_world(x, y);

                    rectangle(
                        tile.colour,
                        [world_x, world_y, self.tile_size, self.tile_size],
                        c.transform,
                        g,
                    );
                }
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("WaveFunctionCollapse", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tile_system = TileSystem::new(512.0, 512.0, 32.0);

    let mut mouse_pos = [0.0, 0.0];

    let mut selected_tile_type = TileType::Water;

    // border pattern wall thing
    for x in 0..tile_system.grid_width {
        tile_system.set_tile(x, 0, Tile::mountain());
        tile_system.set_tile(x, tile_system.grid_height - 1, Tile::mountain());
    }
    for y in 0..tile_system.grid_height {
        tile_system.set_tile(0, y, Tile::mountain());
        tile_system.set_tile(tile_system.grid_width - 1, y, Tile::mountain());
    }

    println!("Tile Controls:");
    println!("1-5        -> Select tile type (Empty/Mountain/Land/Coast/Water)");
    println!("Left click -> place a tile");
    println!("L/S/P      -> Load/Save/Print Configuration");
    println!("C          -> Clear map");
    println!("ESC        -> Exit");
    println!("Current tile: {:?}", selected_tile_type);

    while let Some(event) = window.next() {
        match event {
            Event::Input(Input::Move(Motion::MouseCursor(pos)), _) => {
                mouse_pos = pos;
            }
            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(key),
                    ..
                }),
                _,
            ) => match key {
                Key::D1 => {
                    selected_tile_type = TileType::Empty;
                    println!("Selected: Empty tile");
                }
                Key::D2 => {
                    selected_tile_type = TileType::Mountain;
                    println!("Selected: Mountain tile");
                }
                Key::D3 => {
                    selected_tile_type = TileType::Land;
                    println!("Selected: Land tile");
                }
                Key::D4 => {
                    selected_tile_type = TileType::Coast;
                    println!("Selected: Coast tile");
                }
                Key::D5 => {
                    selected_tile_type = TileType::Water;
                    println!("Selected: Water tile");
                }
                Key::S => {
                    use std::io::{self, Write};
                    print!("Enter name for saved configuration: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    if io::stdin().read_line(&mut input).is_ok() {
                        let name = input.trim().to_string();
                        if !name.is_empty() {
                            tile_system.save_config(name);
                        }
                    }
                }
                Key::L => {
                    use std::io::{self, Write};
                    tile_system.list_configs();
                    print!("Enter name of configuration to load: ");
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    if io::stdin().read_line(&mut input).is_ok() {
                        let name = input.trim();
                        tile_system.load_config(name);
                    }
                }
                Key::C => {
                    tile_system.clear_map();
                    println!("Map cleared");
                }
                Key::P => {
                    tile_system.list_configs();
                }
                _ => {}
            },
            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Mouse(MouseButton::Left),
                    ..
                }),
                _,
            ) => {
                if let Some((grid_x, grid_y)) =
                    tile_system.get_tile_at_pos(mouse_pos[1], mouse_pos[0])
                {
                    let tile_to_place = match selected_tile_type {
                        TileType::Empty => Tile::empty(),
                        TileType::Mountain => Tile::mountain(),
                        TileType::Land => Tile::land(),
                        TileType::Coast => Tile::coast(),
                        TileType::Water => Tile::water(),
                    };

                    tile_system.set_tile(grid_x, grid_y, tile_to_place);
                    // println!(
                    //     "Placed {:?} at ({}, {})",
                    //     selected_tile_type, grid_x, grid_y
                    // );
                }
            }

            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Mouse(MouseButton::Right),
                    ..
                }),
                _,
            ) => {
                if let Some((grid_x, grid_y)) =
                    tile_system.get_tile_at_pos(mouse_pos[1], mouse_pos[0])
                {
                    let tile_to_fill = match selected_tile_type {
                        TileType::Empty => Tile::empty(),
                        TileType::Mountain => Tile::mountain(),
                        TileType::Land => Tile::land(),
                        TileType::Coast => Tile::coast(),
                        TileType::Water => Tile::water(),
                    };

                    tile_system.fill_to_border(grid_x, grid_y, tile_to_fill);
                    println!(
                        "Filled {:?} at ({}, {})",
                        selected_tile_type, grid_x, grid_y
                    );
                }
            }

            //TODO: write a control for loading from and saving to file, test.
            Event::Loop(_) => {
                window.draw_2d(&event, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    tile_system.render(c, g);
                });
            }
            _ => {}
        }
    }
}
