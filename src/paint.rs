use crate::game::Position;

use crate::LENGTH;
use crate::BORDER;
use crate::BLOB_SIZE;


#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Color {
    Blue = color(0, 127, 255),
    White = color(255, 255, 255),
    Black = color(0,0,0),
}

pub fn create_boarder(buffer: &mut Vec<u32>) {


    for x in 0..LENGTH {    
        for y in 0..LENGTH {
            let is_border =
                x < BORDER ||
                x >= LENGTH - BORDER ||
                y < BORDER ||
                y >= LENGTH - BORDER;

            if is_border {
                paint(buffer, &Position::new(x,y), Color::Blue);
            }
        }
    }
}

pub fn paint_blob(buffer: &mut Vec<u32>, center: &Position, color: Color) {
    for x in center.x-BLOB_SIZE..center.x+BLOB_SIZE {
        for y in center.y-BLOB_SIZE..center.y+BLOB_SIZE {

            paint(buffer, &Position::new(x,y), color);
        }
    }
} 


pub fn paint(buffer: &mut Vec<u32>, position: &Position, color: Color) {
    buffer[(position.y * LENGTH) + position.x] = color as u32;
}

pub const fn color(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}
