//! Typedefs for input shortcuts.
use ggez::event::*;
use ggez_goodies::input;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Select,
    Back,
    Menu,
    Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Vertical,
    Horizontal,
}

pub type Binding = input::InputBinding<Axis, Button>;
pub type Event = input::InputEffect<Axis, Button>;
pub type State = input::InputState<Axis, Button>;

// Create the default keybindings for our input state
pub fn create_input_binding() -> input::InputBinding<Axis, Button> {
    input::InputBinding::new()
        .bind_key_to_axis(KeyCode::Up, Axis::Vertical, false)
        .bind_key_to_axis(KeyCode::Down, Axis::Vertical, true)
        .bind_key_to_axis(KeyCode::Left, Axis::Horizontal, false)
        .bind_key_to_axis(KeyCode::Right, Axis::Horizontal, true)
        .bind_key_to_button(KeyCode::C, Button::Select)
        .bind_key_to_button(KeyCode::X, Button::Back)
        .bind_key_to_button(KeyCode::Z, Button::Menu)
        .bind_key_to_button(KeyCode::Escape, Button::Quit)
}
