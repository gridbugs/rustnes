pub const DISPLAY_WIDTH: usize = 256;
pub const DISPLAY_HEIGHT: usize = 240;
pub const NUM_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub trait Frame {
    fn set_pixel(&mut self, x: usize, y: usize, colour: u8);
}
