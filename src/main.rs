mod map;

use ggez::*;
use ggez::graphics::{DrawMode, MeshBuilder, Mesh, Color, Text, Rect};
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::input;
use ggez::{nalgebra as na};
use map::Map;

const SCREEN_SIZE: (f32, f32) = (1200.0, 800.0);

struct State {
    map: Map,
    mouse: Mouse,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let map = Map::new(ctx);
        let mouse = Default::default();

        Ok(
            Self {
                map,
                mouse
            }
        )
    }
}

#[derive(Default)]
struct Mouse {
    x: f32,
    y: f32,
    mouse_down: bool,
}

impl Mouse {
    fn grid_position(&self) -> na::Point2<u32> {
        na::Point2::new(((self.x - 10.0) / map::TILE_SIZE) as u32, ((self.y - 25.0) / map::TILE_SIZE) as u32)
    }

    fn relative_position(&self) -> mint::Point2<f32> {
        mint::Point2 { x: self.x, y: self.y }
    }

    fn set_position(&mut self, pos: mint::Point2<f32>) {
        self.x = pos.x;
        self.y = pos.y;
    }
}




impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left) {
            if input::mouse::position(ctx) != self.mouse.relative_position() {
                self.mouse.set_position(input::mouse::position(ctx));
                let mouse_position = self.mouse.grid_position();
                println!("button pressed x: {}, y: {}", mouse_position.x, mouse_position.y);
                self.map.set_selected_tile(ctx, mouse_position);
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps as u32));

        self.map.render(ctx);
        graphics::draw(ctx, &fps_display, (na::Point2::new(10.0, 5.0), graphics::WHITE),)?;
       
        graphics::present(ctx)?;
        Ok(())

    }
}

fn main() {

    let c = conf::Conf::new();

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .conf(c)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();

    let state = &mut State::new(ctx).unwrap();

    event::run(ctx, event_loop, state).unwrap();

}
