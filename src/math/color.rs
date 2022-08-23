use crate::render::ogl::Uniform;

use super::vector::vec3;

pub struct Color {
    pub format: Format,
}

impl Color {
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            format: Format::RGB { r, g, b },
        }
    }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            format: Format::RGBA { r, g, b, a },
        }
    }

    // pub fn asdad(&self) -> u32 {
    //     match self.format {
    //         Format::RGBA { r, g, b, a } => {
    //             ((r as u32) << 24u32) + ((g as u32) << 16u32) + ((b as u32) << 8u32) + a as u32
    //         }
    //         Format::RGB { r, g, b } => {
    //             ((r as u32) << 24u32) + ((g as u32) << 16u32) + ((b as u32) << 8u32) + 255
    //         }
    //         Format::SRGB => 0,
    //     }
    // }
}

pub enum Format {
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    RGB { r: u8, g: u8, b: u8 },
    SRGB,
}

impl Uniform for Color {
    fn apply_uniform(&mut self, loc: i32) {
        unsafe {
            match self.format {
                Format::RGBA { r, g, b, a } => {
                    let v = vec3(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32);
                    gl::Uniform3fv(loc, 1, v.as_array().as_ptr());
                }
                Format::RGB { r, g, b } => {
                    let v = vec3(r as f32 / 255f32, g as f32 / 255f32, b as f32 / 255f32);
                    gl::Uniform3fv(loc, 1, v.as_array().as_ptr());
                }
                Format::SRGB => todo!(),
            }
        }
    }
}
