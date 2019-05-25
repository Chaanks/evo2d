//! specs systems.
use crate::components::*;
use crate::resources::*;
use crate::types::*;
use crate::map;
use specs::{self, Join};


pub struct NetworkSystem;

impl<'a> specs::System<'a> for NetworkSystem {
    type SystemData = (
        specs::ReadStorage<'a, Transform>,
        specs::WriteStorage<'a, Connection>,
    );

    fn run(&mut self, (pos, mut con): Self::SystemData) {
        for (pos, con) in (&pos, &mut con).join() {
            let data  = format!("{}:{}", pos.grid_position.x, pos.grid_position.y);
            con.socket.send_data(data);
        }
    }
}

// draw directly in level for the moment because we need a ctx
pub struct RenderSystem;

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (
        specs::ReadStorage<'a, Transform>,
        specs::ReadStorage<'a, Graphic>,
    );

    fn run(&mut self, (pos, mesh): Self::SystemData) {
        for (pos, mesh) in (&pos, &mesh).join() {

        }
    }
}

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Transform>,
        specs::ReadStorage<'a, Motion>,
    );

    fn run(&mut self, (mut pos, motion): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion) in (&mut pos, &motion).join() {
            pos.position += motion.velocity;
            let x = (pos.grid_position.x as f32 + motion.velocity.x as f32) as i32;
            let y = (pos.grid_position.y as f32 + motion.velocity.y as f32) as i32;

            if  x > 0 && x < (map::CELL_NUMBER - 1) as i32 {
                pos.grid_position.x = x as u32;
            
            }
            if  y > 0 && y < (map::CELL_NUMBER - 1) as i32 {
                pos.grid_position.y = y as u32;
            
            }

            pos.position = map::Map::relative_position(pos.grid_position);
            pos.position.x += (map::TILE_SIZE / 2.0 - pos.size / 2.0) - 3.0;
            pos.position.y += (map::TILE_SIZE / 2.0 - pos.size / 2.0) - 3.0;
            
        }
    }
}


pub struct InputSystem;

impl<'a> specs::System<'a> for InputSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::WriteStorage<'a, Motion>,
        specs::ReadStorage<'a, ArrowController>,
    );

    fn run(&mut self, (input, mut motion, controller): Self::SystemData) {
        for (motion, _controller) in (&mut motion, &controller).join() {
                motion.velocity.x = input.horizontal;
                motion.velocity.y = input.vertical;
        }
    }
}


pub struct SelectionSystem;

impl<'a> specs::System<'a> for SelectionSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::Read<'a, Input>,
        specs::Write<'a, Selected>,
        specs::ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entity, input, mut selection, transform): Self::SystemData) {
        //println!("Mouse position: {:?}", input.mouse_position);
        let mouse_pos = na::Point2::new(input.mouse_position.0, input.mouse_position.1);
        for (entity, transform) in (&*entity, &transform).join() {
            if map::Map::grid_position(mouse_pos) == transform.grid_position {
                        if selection.player == Some(entity) {
                            if selection.isClicked {
                                if input.mouse_pressed {
                                    selection.isClicked = false;
                                }
                            } else if input.mouse_pressed {
                                selection.isClicked = true
                            }

                            return;

                        } else if selection.player != Some(entity) {
                            if selection.isClicked && input.mouse_pressed {
                                selection.player = Some(entity);
                            } else if !selection.isClicked {
                                selection.player = Some(entity);
                            }
                                return;
                        }
                    }
                }

        if !selection.isClicked {
            selection.player = None;
        }

    }
}