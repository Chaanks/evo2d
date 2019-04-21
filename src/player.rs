use specs::{Component, VecStorage, System, ReadStorage};
use ggez::nalgebra as na;


#[derive(Debug)]
pub struct Transform {
    pub position: na::Point2<f32>,
    pub rotation: f32,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Transform>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}