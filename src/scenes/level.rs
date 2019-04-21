use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
use specs::{self, Join};
use warmy;
use ggez::Context;
use imgui::*;

use crate::components as c;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::systems::*;
use crate::world::World;


pub struct LevelScene {
    done: bool,
    kiwi: warmy::Res<resources::Image>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let kiwi = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/kiwi.png"), ctx)
            .unwrap();

        let dispatcher = Self::register_systems();
        LevelScene {
            done,
            kiwi,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
            .build()
    }
}


impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pos = gameworld.specs_world.read_storage::<c::Transform>();
        for p in pos.join() {
            graphics::draw(
                ctx,
                &(self.kiwi.borrow().0),
                graphics::DrawParam::default().dest(p.position),
            )?;
        }
        
        Ok(())
    }

    fn draw_ui(&mut self, ctx: &mut Context, ui: &mut Ui) {
        // Window
        ui.window(im_str!("Hello world"))
            .size((300.0, 100.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.imgui().mouse_pos();
                ui.text(im_str!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos.0,
                    mouse_pos.1
                ));
        });        
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}