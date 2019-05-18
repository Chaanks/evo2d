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
            con.socket.send_data("yo");
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

            if  pos.position.x < map::TILE_SIZE + map::WIDTH_OFFSET  + pos.size {
                pos.position.x -= motion.velocity.x;
            
            } else if  pos.position.x > map::SIZE - map::TILE_SIZE + map::WIDTH_OFFSET - pos.size {
                pos.position.x -= motion.velocity.x;
            }

            if pos.position.y < map::TILE_SIZE + map::HEIGHT_OFFSET  + pos.size {
                pos.position.y -= motion.velocity.y;
            } else if  pos.position.y > map::SIZE - map::TILE_SIZE + map::HEIGHT_OFFSET - pos.size {
                pos.position.y -= motion.velocity.y;
            }
            
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
        for (entity, transform) in (&*entity, &transform).join() {
                if input.mouse_position.0 >= transform.position.x - transform.size && input.mouse_position.0 < transform.position.x + transform.size {
                    if input.mouse_position.1 >= transform.position.y - transform.size && input.mouse_position.1 < transform.position.y + transform.size {

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
        }

        if !selection.isClicked {
            selection.player = None;
        }

    }
}