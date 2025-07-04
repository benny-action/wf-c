use std::usize;

use piston_window::*;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

#[derive(Clone, Debug)]
pub struct Tile {
    pub colour: [f32; 4],
    pub tile_type: TileType,
    pub visible: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Floor,
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
    pub fn wall() -> Self {
        Tile::new(TileType::Wall, [0.5, 0.5, 0.5, 1.0])
    }
    pub fn floor() -> Self {
        Tile::new(TileType::Floor, [0.8, 0.7, 0.6, 1.0])
    }
    pub fn water() -> Self {
        Tile::new(TileType::Water, [0.2, 0.4, 0.8, 1.0])
    }
}

pub struct TileSystem {
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: f64,
    pub grid_width: usize,
    pub grid_height: usize,
    pub window_width: f64,
    pub window_height: f64,
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

    pub fn grid_to_world(&self, grid_x: usize, grid_y: usize) -> (f64, f64) {
        (
            grid_x as f64 * self.tile_size,
            grid_y as f64 * self.tile_size,
        )
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

    // border pattern wall thing
    for x in 0..tile_system.grid_width {
        tile_system.set_tile(x, 0, Tile::wall());
        tile_system.set_tile(x, tile_system.grid_height - 1, Tile::wall());
    }
    for y in 0..tile_system.grid_height {
        tile_system.set_tile(0, y, Tile::wall());
        tile_system.set_tile(tile_system.grid_width - 1, y, Tile::wall());
    }

    while let Some(event) = window.next() {
        match event {
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
