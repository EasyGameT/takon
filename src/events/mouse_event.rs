use winit::event::MouseButton;

use super::Event;

pub struct MouseMoved {
    pub x: f32,
    pub y: f32,
}
impl MouseMoved {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Event for MouseMoved {}

pub struct MouseScrolled {
    pub x: f32,
    pub y: f32,
}
impl MouseScrolled {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Event for MouseScrolled {}

pub struct MouseButtonPressed {
    pub button: MouseButton,
}
impl MouseButtonPressed {
    pub fn new(button: MouseButton) -> Self {
        Self { button }
    }
}
impl Event for MouseButtonPressed {}

pub struct MouseButtonReleased {
    pub button: MouseButton,
}
impl MouseButtonReleased {
    pub fn new(button: MouseButton) -> Self {
        Self { button }
    }
}
impl Event for MouseButtonReleased {}
