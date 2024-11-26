use crate::log::Log;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Log::new();

        Self {}
    }

    pub fn run(&mut self) {}
}
