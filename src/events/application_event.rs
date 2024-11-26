use super::event::Event;

pub struct WindowResized {
    pub width: u32,
    pub height: u32,
}
impl WindowResized {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl Event for WindowResized {}

pub struct WindowClosed;
impl WindowClosed {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for WindowClosed {}

pub struct AppTicked;
impl AppTicked {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for AppTicked {}

pub struct AppUpdated;
impl AppUpdated {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for AppUpdated {}

pub struct AppRendered;
impl AppRendered {
    pub fn new() -> Self {
        Self {}
    }
}
impl Event for AppRendered {}
