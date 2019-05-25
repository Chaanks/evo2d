use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
use specs::{self, Join, world::Builder};
use warmy;
use ggez::Context;
use imgui::*;

use crate::types::na;
use crate::components;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::systems::*;
use crate::world::World;
use crate::network;
use crate::map;
use crate::vision::*;


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


        // add input ressource
        world
            .specs_world
            .add_resource(resources::Input::new());
        world
            .specs_world
            .add_resource(resources::Selected::new());
        world
            .specs_world
            .add_resource(map::Map::new(ctx));
        

        let dispatcher = Self::register_systems();

        LevelScene {
            done,
            kiwi,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(InputSystem, "sys_input", &[])
            .with(NetworkSystem, "sys_network", &[])
            .with(MovementSystem, "sys_movement", &["sys_input"])
            .with(SelectionSystem, "sys_selection", &[])
            .build()
    }

    fn new_agent(world: &mut World, ctx: &mut ggez::Context) {
        // Make a test entity.
        let size = 6.0;
        let mesh = &mut graphics::MeshBuilder::new();
        mesh.circle(graphics::DrawMode::fill(), ggez::mint::Point2{x: 0.0, y: 0.0}, size, 0.001, graphics::Color::new(1.0, 0.0, 0.494, 0.9));
        mesh.circle(graphics::DrawMode::stroke(2.0), ggez::mint::Point2{x: 0.0, y: 0.0}, size, 0.001, graphics::Color::new(0.0, 0.0, 0.0, 0.9));
        let circle = mesh.build(ctx).unwrap();

        world
            .specs_world
            .create_entity()
            .with(components::Graphic { mesh: circle })
            .with(components::Transform { position: na::Point2::new(300.0, 300.0), grid_position: map::Map::grid_position(na::Point2::new(300.0, 300.0)), rotation: 0.0, size: size })
            .with(components::Motion {
                velocity: na::Vector2::new(0.0, 0.0),
                acceleration: na::Vector2::new(0.0, 0.0),
            })
            .with(components::ArrowController)
            .with(components::Connection::default())
            .with(components::Vision::default())
            .build();
    }
}


impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> scenes::Switch {
        {
            let mut input_state = gameworld.specs_world.write_resource::<resources::Input>();
            input_state.vertical = gameworld.input.get_axis_raw(input::Axis::Vertical);
            input_state.horizontal = gameworld.input.get_axis_raw(input::Axis::Horizontal);
            input_state.mouse_position = (ggez::input::mouse::position(ctx).x, ggez::input::mouse::position(ctx).y);
            if ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) {
                input_state.mouse_pressed = true
            } else {
                input_state.mouse_pressed = false
            }

            let mut map = gameworld.specs_world.write_resource::<map::Map>();
            let mouse_pos = na::Point2::new(input_state.mouse_position.0, input_state.mouse_position.1);
            if map::Map::on_map(mouse_pos) {
                let mouse_position = map::Map::grid_position(mouse_pos);
                //println!("button pressed x: {}, y: {}", mouse_position.x, mouse_position.y);
                map.set_selected_tile(ctx, mouse_position);
            }

            let selected_entity = gameworld.specs_world.read_resource::<resources::Selected>();
            let positions = gameworld.specs_world.read_storage::<components::Transform>();
            let visions = gameworld.specs_world.read_storage::<components::Vision>();
            match selected_entity.player {
                Some(e) =>  {
                    let pos = positions.get(e).unwrap().position;
                    let grid_pos = map::Map::grid_position(pos);
                    let vision = visions.get(e).unwrap();
                    let grid_view = map::Map::grid_view(&vision.view, grid_pos);
                    map.set_selected_player_view(ctx, Some(grid_view));
                }

                None => map.set_selected_player_view(ctx, None),
            }


        }
        self.dispatcher.dispatch(&gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let map = gameworld.specs_world.read_resource::<map::Map>();
        map.render(ctx);
        let pos = gameworld.specs_world.read_storage::<components::Transform>();
        let mesh = gameworld.specs_world.read_storage::<components::Graphic>();
        for (p, m) in (&pos, &mesh).join() {
            graphics::draw(
                ctx,
                &m.mesh,
                graphics::DrawParam::default().dest(p.position),
            )?;
        }
        
        Ok(())
    }

    fn draw_ui(&mut self, gameworld: &mut World, ctx: &mut Context, ui: &mut Ui) {
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

                ui.separator();
                let vert = gameworld.input.get_axis_raw(input::Axis::Vertical);
                let hori = gameworld.input.get_axis_raw(input::Axis::Horizontal);
                ui.text(im_str!(
                    "input Axis: ({},{})",
                    vert,
                    hori
                ));


                ui.separator();
                if ui.button(im_str!("New Entity"), ImVec2::new(100.0, 25.0)) {
                    Self::new_agent(gameworld, ctx);
                }

                ui.separator();
                let selected_entity = gameworld.specs_world.read_resource::<resources::Selected>();
                let positions = gameworld.specs_world.read_storage::<components::Transform>();
                let mut visions = gameworld.specs_world.write_storage::<components::Vision>();
                match selected_entity.player {
                    Some(e) =>  {
                        // switch view
                        ui.text(im_str!("Switch Vision"));
                        let mut vision = visions.get_mut(e).unwrap();
                        if ui.button(im_str!("Square"), ImVec2::new(100.0, 25.0)) {
                            vision.update_view(ViewShape::SQUARE);
                        }
                        if ui.button(im_str!("Diamond"), ImVec2::new(100.0, 25.0)) {
                            vision.update_view(ViewShape::DIAMOND);
                        }
                        if ui.button(im_str!("Triangle"), ImVec2::new(100.0, 25.0)) {
                            vision.update_view(ViewShape::TRIANGLE);
                        }
                        ui.separator();

                        let pos = positions.get(e).unwrap().position;
                        ui.text_colored(ImVec4::new(1.0, 1.0, 1.0, 1.0),im_str!("Player:  XXX "));
                        ui.separator();
                        ui.text(im_str!("Position ({:.1} {:.1})", pos.x, pos.y));
                        ui.with_color_var(ImGuiCol::PlotHistogram, ImVec4::new(0.0, 1.0, 0.0, 1.0), || {
                            ui.text(im_str!("Health"));
                            ui.progress_bar(0.8)
                                .size((100.0, 15.0))
                                .overlay_text(im_str!("80/100"))           
                                .build();
                        });

                            ui.text(im_str!("Food"));
                            ui.progress_bar(0.8)
                                .size((100.0, 15.0))
                                .overlay_text(im_str!("80/100"))           
                                .build();              

                        ui.with_color_var(ImGuiCol::PlotHistogram, ImVec4::new(0.0, 0.0, 1.0, 1.0), || {
                            ui.text(im_str!("Water"));
                            ui.progress_bar(0.2)
                                .size((100.0, 15.0))
                                .overlay_text(im_str!("20/100"))           
                                .build();
                        });
                        ui.separator();
                    },
                    None => {}, 
                }
                

               

        });
          
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, started: bool) {
        println!("Input: {:?}", ev);

        gameworld.input.update_effect(ev, started);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }

    }
}