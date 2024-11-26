use winit::keyboard::Key;

use super::Event;

pub struct KeyPressed {
    key: Key,
}
impl KeyPressed {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}
impl Event for KeyPressed {}

pub struct KeyReleased {
    key: Key,
}
impl KeyReleased {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}
impl Event for KeyReleased {}
