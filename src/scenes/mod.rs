use ggez_goodies::scene;

use crate::input;
use crate::world::World;

use ggez::Context;
use imgui::Ui;

pub mod level;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<World, input::Event>;
pub type Stack = scene::SceneStack<World, input::Event>;
