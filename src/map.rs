use ggez::*;
use ggez::graphics::{DrawMode, MeshBuilder, Mesh, Color, Text, Rect};
use ggez::{nalgebra as na};

pub const SIZE: f32 = 750.0;
pub const CELL_NUMBER: u32 = 20;
pub const TILE_SIZE: f32 = SIZE / CELL_NUMBER as f32;

pub const TILESET: [i32; 400] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const GREEN: (f32, f32, f32) = (0.0, 0.77, 0.1);
const BLUE: (f32, f32, f32) = (0.0, 0.33, 0.77);
const GRAY: (f32, f32, f32) = (0.48, 0.48, 0.48);

#[derive(Debug)]
enum Tile {
    Undefined = -1,
    Wall = 0,
    Grass = 1,
    Water = 2,
}

pub struct Map {
    tiles: Vec<Tile>,
    grid: Mesh,
    bg: Mesh,
    selected_tile: Mesh,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map{

        //create from toml file
        /*
        let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

        let package_info: Value = toml::from_str(contents.as_str()).unwrap();
        println!("{}", package_info);
        */

        let mut tiles = vec!();

        for tile in TILESET.iter() {
            match tile {
                -1 => tiles.push(Tile::Undefined),
                0 => tiles.push(Tile::Wall),
                1 => tiles.push(Tile::Grass),
                2 => tiles.push(Tile::Water),
                _ => println!("Ain't special"),
            }
        }

        let grid = Self::grid(ctx);
        let bg =  Self::background(&tiles, ctx);
        let selected_tile = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 0.0 , 0.0),
            graphics::WHITE,
        ).unwrap();

        Self {
            tiles,
            grid,   
            bg,
            selected_tile,
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.bg, (na::Point2::new(10.0, 25.0),)).unwrap();
        graphics::draw(ctx, &self.grid, (na::Point2::new(10.0, 25.0),)).unwrap();
        graphics::draw(ctx, &self.selected_tile, (na::Point2::new(10.0, 25.0),)).unwrap();
    }

    pub fn set_selected_tile(&mut self, ctx: &mut Context, mouse_position: na::Point2<u32>) {
            self.selected_tile = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new((mouse_position.x as f32) * TILE_SIZE + 1.0, (mouse_position.y as f32) * TILE_SIZE + 1.0, TILE_SIZE - 2.0 , TILE_SIZE - 2.0),
            graphics::WHITE,
        ).unwrap();
    }

    fn grid(ctx: &mut Context) -> Mesh{
        let mesh = &mut MeshBuilder::new();

        for i in 0..CELL_NUMBER as u32{
            for j in 0..CELL_NUMBER as u32{
                let start_x = i as f32 * TILE_SIZE;
                let start_y = j as f32 * TILE_SIZE;

                let color= Color::new(0.0, 0.0, 0.0, 1.0);

                mesh.rectangle(DrawMode::stroke(2.0),
                            graphics::Rect::new(start_x, start_y, TILE_SIZE, TILE_SIZE),
                            color);
            }

        }

        mesh.build(ctx).unwrap()

    }

    fn background(tiles: &Vec<Tile>, ctx: &mut Context) -> Mesh{
        let mesh = &mut MeshBuilder::new();

        for i in 0..CELL_NUMBER as u32{
            for j in 0..CELL_NUMBER as u32{
                let start_x = i as f32 * TILE_SIZE;
                let start_y = j as f32 * TILE_SIZE;
                

                let index = i * CELL_NUMBER + j;
                let current = &tiles[index as usize];


                let color: Color;
                match current {
                    Tile::Undefined => color = Color::new(0.0, 0.0, 0.0, 1.0),
                    Tile::Grass => color = Color::from(GREEN),
                    Tile::Wall => color = Color::from(GRAY),
                    Tile::Water => color = Color::from(BLUE),
                    _ => color = Color::new(0.0, 0.0, 0.0, 1.0),
                }

                println!("i:{}, j:{}, current{:?}, color:{:?}",i, j, current, color);
                mesh.rectangle(DrawMode::fill(),
                            graphics::Rect::new(start_x, start_y, TILE_SIZE, TILE_SIZE),
                            color);
            }

        }

        mesh.build(ctx).unwrap()

    }
}