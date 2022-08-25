use std::{
    ops::{Deref, DerefMut},
    sync::mpsc::Receiver,
};

pub use glfw::Context;
pub use glfw::Key;
pub use glfw::WindowEvent;
use glfw::{Action, FlushedMessages, Window as GlfwWindow, WindowHint, WindowMode, FAIL_ON_ERRORS};

use crate::math::color::{Color, Format};

pub struct FrameData {
    pub time: f64,
    pub events: WindowEvent,
}

pub struct EventIter<'a> {
    events: &'a mut FlushedMessages<'a, (f64, WindowEvent)>,
}

#[derive(Debug)]
pub struct Window {
    handle: GlfwWindow,
    events: Receiver<(f64, WindowEvent)>,
}

impl Deref for Window {
    type Target = GlfwWindow;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new("QEngine", 800, 600)
    }
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(FAIL_ON_ERRORS).expect("Failed to init glfw");
        glfw.window_hint(WindowHint::ContextVersionMajor(3));
        glfw.window_hint(WindowHint::ContextVersionMinor(3));

        let (mut w, e) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
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
            Ok(data) => {
                let (dt, event) = data;

                if let WindowEvent::Size(x, y) = event {
                    unsafe {
                        gl::Viewport(0, 0, x, y);
                        self.clear(Color::GREEN);
                        self.swap_buffers();
                    }
                }

                Some(FrameData {
                    time: dt,
                    events: event,
                })
            }
            _ => {
                self.handle.glfw.poll_events();
                None
            }
        }
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            match color.format {
                Format::RGB { r, g, b } => {
                    gl::ClearColor(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);
                }
                Format::RGBA { r, g, b, a } => todo!(),
                Format::SRGB => todo!(),
            }

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

pub trait InputEvents {
    fn is_key_pressed(&self, key: Key) -> bool;
}

impl InputEvents for FrameData {
    fn is_key_pressed(&self, k: Key) -> bool {
        match self.events {
            WindowEvent::Key(key, code, act, mods) => k == key && act == Action::Press,
            _ => false,
        }
    }
}
