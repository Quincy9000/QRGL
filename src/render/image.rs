// use std::io::Read;

// use crate::render::image::image_formats::headers::JpgHeader;

// use image::*;

// pub struct Image {
//     data: Vec<u8>,
//     name: String,
// }

// impl From<&str> for Image {
//     fn from(s: &str) -> Self {
//         let bytes = image::open(s)
//             .expect("Could not find the file")
//             .as_mut_rgb8()
//             .unwrap();

//         // let mut data = Vec::new();
//         // std::fs::File::open(s)
//         //     .unwrap()
//         //     .read_to_end(&mut data)
//         //     .unwrap();

//         let mut first = &data[0..2];

//         use image_formats::*;

//         match first {
//             _ if first.starts_with(&JPG) => {
//                 let jpg = JpgHeader::new(&data);
//                 println!("Its a jpg: {jpg:?}");

//                 let last = &data[data.len() - 2..data.len()];
//                 println!("{last:?}");
//             }
//             _ if first.starts_with(&BMP) => {
//                 println!("Its a bmp");
//             }
//             _ if first.starts_with(&PNG) => {
//                 println!("Its a png");
//             }
//             _ => {}
//         }

//         Self {
//             data,
//             name: s.into(),
//         }
//     }
// }

// #[test]
// fn load_image() {
//     let mut image = Image::from("wall.jpg");
// }

// mod image_formats {
//     pub const JPG: [u8; 2] = [0xFF, 0xD8];
//     pub const BMP: [u8; 2] = [0x42, 0x4D];
//     pub const PNG: [u8; 4] = [0x89, 0x50, 0x4E, 0x47];
//     pub const GIF: [u8; 4] = [0x47, 0x49, 0x46, 0x38];

//     pub mod headers {
//         #[derive(Debug, Clone, Copy)]
//         pub struct JpgHeader {
//             soi: [u8; 2],
//             app0: [u8; 2],
//             len: [u8; 2],
//             id: [u8; 5],
//             vers: [u8; 2],
//             units: u8,
//             xd: [u8; 2],
//             yd: [u8; 2],
//             xt: u8,
//             yt: u8,
//         }

//         impl JpgHeader {
//             pub fn new(bytes: &Vec<u8>) -> Self {
//                 assert!(bytes.starts_with(&super::JPG));
//                 let b = bytes;
//                 // https://www.file-recovery.com/jpg-signature-format.htm
//                 Self {
//                     soi: [b[0], b[1]],
//                     app0: [b[2], b[3]],
//                     len: [b[4], b[5]],
//                     id: [b[6], b[7], b[8], b[9], b[10]],
//                     vers: [b[11], b[12]],
//                     units: b[13],
//                     xd: [b[14], b[15]],
//                     yd: [b[16], b[17]],
//                     xt: b[18],
//                     yt: b[19],
//                 }
//             }
//         }
//     }
// }
