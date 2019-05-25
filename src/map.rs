use ggez::*;
use ggez::graphics::{DrawMode, MeshBuilder, Mesh, Color, Text, Rect};
use ggez::{nalgebra as na};
use specs::Entity;

pub const SIZE: f32 = 750.0;
pub const CELL_NUMBER: u32 = 40;
pub const TILE_SIZE: f32 = SIZE / CELL_NUMBER as f32;
pub const WIDTH_OFFSET: f32 = 10.0;
pub const HEIGHT_OFFSET: f32 = 25.0;

pub const TILESET: [i32; 1600] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
];

const GREEN: (f32, f32, f32) = (0.0, 0.77, 0.1);
const BLUE: (f32, f32, f32) = (0.0, 0.33, 0.77);
const GRAY: (f32, f32, f32) = (0.48, 0.48, 0.48);

#[derive(Debug)]
pub enum Tile {
    Undefined = -1,
    Wall = 0,
    Grass = 1,
    Water = 2,
}

pub struct Cell {
    pub tile: Tile,
    entities: Vec<Entity>,
}

impl Cell {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile,
            entities: Vec::new(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self { Cell::new(Tile::Undefined) }
}


pub struct Map {
    cells: Vec<Cell>,
    bg: Mesh,
    selected_tile: Option<Mesh>,
    selected_player: Option<Mesh>,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map{
        let mut cells = vec!();

        for tile in TILESET.iter() {
            match tile {
                -1 => cells.push(Cell::new(Tile::Undefined)),
                0 => cells.push(Cell::new(Tile::Wall)),
                1 => cells.push(Cell::new(Tile::Grass)),
                2 => cells.push(Cell::new(Tile::Water)),
                _ => println!("Ain't special"),
            }
        }
        let bg =  Self::background(&cells, ctx);
        Self {
            cells,
            bg,
            selected_tile: None,
            selected_player: None,
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.bg, (na::Point2::new(WIDTH_OFFSET, HEIGHT_OFFSET),)).unwrap();
        match &self.selected_tile {
            Some(e) => graphics::draw(ctx, e, (na::Point2::new(WIDTH_OFFSET, HEIGHT_OFFSET),)).unwrap(),
            None => {},
        
        }
        match &self.selected_player {
            Some(e) => graphics::draw(ctx, e, (na::Point2::new(WIDTH_OFFSET, HEIGHT_OFFSET),)).unwrap(),
            None => {},
        
        }
    }

    pub fn set_selected_tile(&mut self, ctx: &mut Context, mouse_position: na::Point2<u32>) {
            self.selected_tile = Some(graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new((mouse_position.x as f32) * TILE_SIZE + 0.5, (mouse_position.y as f32) * TILE_SIZE + 0.5, TILE_SIZE - 1.0 , TILE_SIZE - 1.0),
            Color::new(1.0, 1.0, 1.0, 0.1),
        ).unwrap());
    }

    pub fn set_selected_player_view(&mut self, ctx: &mut Context, grid_pos: Option<Vec<na::Point2<u32>>>) {
        match grid_pos {
            Some(positions) => {
                let mesh = &mut MeshBuilder::new();
                let color = Color::new(1.0, 1.0, 1.0, 0.3);
                for pos in positions {
                    mesh.rectangle(DrawMode::fill(),
                                graphics::Rect::new((pos.x as f32) * TILE_SIZE + 0.5, (pos.y as f32) * TILE_SIZE + 0.5, TILE_SIZE - 1.0 , TILE_SIZE - 1.0),
                                color);
                }

                self.selected_player = Some(mesh.build(ctx).unwrap());
            },

            None => self.selected_player = None,
        }
    }

    fn background(cells: &Vec<Cell>, ctx: &mut Context) -> Mesh{
        let mesh = &mut MeshBuilder::new();

        for i in 0..CELL_NUMBER as u32{
            for j in 0..CELL_NUMBER as u32{
                let start_x = i as f32 * TILE_SIZE;
                let start_y = j as f32 * TILE_SIZE;
                

                let index = i * CELL_NUMBER + j;
                let current = &cells[index as usize].tile;


                let mut color: Color;
                match current {
                    Tile::Undefined => color = Color::new(0.0, 0.0, 0.0, 1.0),
                    Tile::Grass => color = Color::from(GREEN),
                    Tile::Wall => color = Color::from(GRAY),
                    Tile::Water => color = Color::from(BLUE),
                    _ => color = Color::new(0.0, 0.0, 0.0, 1.0),
                }

                //println!("i:{}, j:{}, current{:?}, color:{:?}",i, j, current, color);
                mesh.rectangle(DrawMode::fill(),
                            graphics::Rect::new(start_x, start_y, TILE_SIZE, TILE_SIZE),
                            color);

                color = Color::new(0.0, 0.0, 0.0, 1.0);

                mesh.rectangle(DrawMode::stroke(1.0),
                            graphics::Rect::new(start_x, start_y, TILE_SIZE, TILE_SIZE),
                            color);
            }

        }

        mesh.build(ctx).unwrap()

    }

    pub fn on_map(pos: na::Point2<f32>) -> bool {
        if pos.x < 0.0 + WIDTH_OFFSET || pos.y < 0.0 + HEIGHT_OFFSET || pos.x > SIZE + WIDTH_OFFSET|| pos.y > SIZE + HEIGHT_OFFSET {
            return false;
        }
        true
    }

    pub fn grid_position(pos: na::Point2<f32>) -> na::Point2<u32> {
        na::Point2::new(((pos.x - 10.0) / TILE_SIZE) as u32, ((pos.y - 25.0) / TILE_SIZE) as u32)
    }

    pub fn relative_position(pos: na::Point2<u32>) -> na::Point2<f32> {
        na::Point2::new(((pos.x as f32) * TILE_SIZE) + WIDTH_OFFSET + 6.0, ((pos.y as f32) * TILE_SIZE) + HEIGHT_OFFSET + 6.0) 
    }

    pub fn grid_view(vision: &Vec<(i32, i32)>, position: na::Point2<u32>) -> Vec<na::Point2<u32>> {
        let mut views = Vec::new();
        for view in vision {
            
            let tile: na::Point2<u32> = na::Point2::new((view.0 + position.x as i32) as u32, (view.1 + position.y as i32) as u32);
            if tile.x >= CELL_NUMBER  as u32 || tile.y >= CELL_NUMBER as u32 {
                continue;
            }
            
            //println!("position: {}, view: {:?}, tile: {:?}", position, view, tile);
            views.push(tile);
        }

        views
    }

}