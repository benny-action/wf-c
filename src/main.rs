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

struct TileSystem {}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("WaveFunctionCollapse", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });
    }
}
