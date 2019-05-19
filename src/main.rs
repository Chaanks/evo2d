mod map;
mod imgui_wrapper;
mod input;
mod resources;
mod scenes;
mod systems;
mod types;
mod util;
mod world;
mod components;
mod network;

use std::env;
use std::path;
use ggez::*;
use ggez::graphics::{DrawMode, MeshBuilder, Mesh, Color, Text, Rect};
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::{nalgebra as na};
use map::Map;



use crate::imgui_wrapper::ImGuiWrapper;

const SCREEN_SIZE: (f32, f32) = (1200.0, 800.0);

struct State {
    mouse: Mouse,
    imgui_wrapper: ImGuiWrapper,
    scenes: scenes::Stack,
    input_binding: input::Binding,
}

impl State {
    fn new(ctx: &mut Context, resource_path: &path::Path) -> GameResult<State> {
        let mouse = Default::default();
        let imgui_wrapper = ImGuiWrapper::new(ctx);

        let world = world::World::new(resource_path);
        let mut scenestack = scenes::Stack::new(ctx, world);
        let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);

        Ok(
            Self {
                mouse,
                imgui_wrapper,
                input_binding: input::create_input_binding(),
                scenes: scenestack,
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
    fn relative_position(&self) -> mint::Point2<f32> {
        mint::Point2 { x: self.x, y: self.y }
    }

    fn get_position(&self) -> na::Point2<f32> {
        na::Point2::new(self.x, self.y)
    }

    fn set_position(&mut self, pos: mint::Point2<f32>) {
        self.x = pos.x;
        self.y = pos.y;
    }

}




impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) {
            if ggez::input::mouse::position(ctx) != self.mouse.relative_position(){
                self.mouse.set_position(ggez::input::mouse::position(ctx));
            }
        }

        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update(ctx);
        }
        self.scenes.world.resources.sync(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps as u32));

        graphics::draw(ctx, &fps_display, (na::Point2::new(900.0, 20.0), graphics::WHITE),)?;
        self.scenes.draw(ctx);
        self.imgui_wrapper.render_scene_ui(ctx, &mut self.scenes);
       
        graphics::present(ctx)?;
        Ok(())

    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: event::KeyMods, repeat: bool,) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: event::KeyMods,) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, false);
        }
    }

    fn mouse_motion_event(&mut self,  _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }


    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32 ) {
        self.imgui_wrapper.update_mouse_down((
        button == MouseButton::Left,
        button == MouseButton::Right,
        button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_down((
            match button {
                MouseButton::Left => false,
                _ => true,
            },
            match button {
                MouseButton::Right => false,
                _ => true,
            },
            match button {
                MouseButton::Middle => false,
                _ => true,
            },
        ));
    }
}

fn main() {

    util::setup_logging();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    println!("Resource dir: {:?}", resource_dir);

    let cb = ContextBuilder::new("game-template", "ggez")
        .window_setup(conf::WindowSetup::default().title("game template"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(&resource_dir);
    let (ctx, ev) = &mut cb.build().unwrap();

    let state = &mut State::new(ctx, &resource_dir).unwrap();
    if let Err(e) = event::run(ctx, ev, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }


}
