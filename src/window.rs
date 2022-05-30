use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Receiver,
};

use glfw::*;

pub use glfw::Context;
pub use glfw::Key;
pub use glfw::WindowEvent;

pub struct FrameData {
    pub time: f64,
    pub events: WindowEvent,
}

pub struct EventIter<'a> {
    events: &'a mut FlushedMessages<'a, (f64, WindowEvent)>,
}

#[derive(Debug)]
pub struct Window {
    handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Deref for Window {
    type Target = glfw::Window;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

impl Window {
    pub fn new() -> Self {
        let mut glfw = glfw::init(FAIL_ON_ERRORS).expect("Failed to init glfw");
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));

        let (mut w, e) = glfw
            .create_window(800, 600, "Q", WindowMode::Windowed)
            .expect("Failed to make a window");

        w.make_current();
        w.set_all_polling(true);

        gl::load_with(|s| w.get_proc_address(s));

        Self {
            handle: w,
            events: e,
        }
    }

    pub fn events(&mut self) -> Option<FrameData> {
        match self.events.try_recv() {
            Ok(data) => Some(FrameData {
                time: data.0,
                events: data.1,
            }),
            _ => {
                self.glfw.poll_events();
                None
            }
        }
    }
}

pub trait InputEvents {
    fn is_key_pressed(&self, key: Key) -> bool;
}

impl InputEvents for FrameData {
    fn is_key_pressed(&self, key: Key) -> bool {
        match self.events {
            WindowEvent::Key(key, code, act, mods) => key == key,
            _ => false,
        }
    }
}
