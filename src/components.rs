use crate::types::*;
use crate::network::Client;
use crate::vision::*;
use specs::*;
use specs_derive::*;
use ggez::graphics;


// ///////////////////////////////////////////////////////////////////////
// Components
// ///////////////////////////////////////////////////////////////////////

/// View
#[derive(Component)]
#[storage(VecStorage)]
pub struct Vision {
    pub view: Vec<(i32, i32)>,
}

impl Default for Vision {
    fn default() -> Self {
        Self {
            view: SQUARE_ID.to_vec(),
        }
    }
}

impl Vision {
    pub fn new(shape: ViewShape) -> Self {
        let mut view = vec!();
        match shape {
            ViewShape::SQUARE => view = SQUARE_ID.to_vec(),
            ViewShape::DIAMOND => view = DIAMOND_ID.to_vec(),
            ViewShape::TRIANGLE => view = TRIANGLE_ID.to_vec(),
        }

        Self {
            view,
        }
    }

    pub fn update_view(&mut self, shape: ViewShape) {
        match shape {
            ViewShape::SQUARE => self.view = SQUARE_ID.to_vec(),
            ViewShape::DIAMOND => self.view = DIAMOND_ID.to_vec(),
            ViewShape::TRIANGLE => self.view = TRIANGLE_ID.to_vec(),
        }
    }
}



/// Network
#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Connection {
    pub socket: Client,
}

/// Mesh
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Graphic {
    pub mesh: graphics::Mesh,
}

/// A position in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: na::Point2<f32>,
    pub grid_position: na::Point2<u32>,
    pub rotation: f32,
    pub size: f32,
}

/// Motion in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: na::Vector2<f32>,
    pub acceleration: na::Vector2<f32>,
}

#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct ArrowController;

/// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Player {
    pub name: String,
    pub health: f32,
    pub food: f32,
    pub water: f32,
}

#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Shot {
    pub damage: u32,
}



pub fn register_components(specs_world: &mut World) {
    specs_world.register::<Transform>();
    specs_world.register::<Motion>();
    specs_world.register::<ArrowController>();
    specs_world.register::<Shot>();
    specs_world.register::<Player>();
    specs_world.register::<Graphic>();
    specs_world.register::<Connection>();
    specs_world.register::<Vision>();
}