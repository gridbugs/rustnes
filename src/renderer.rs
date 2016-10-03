pub trait Frame {
    fn set_pixel(&mut self, x: usize, y: usize, colour: u8);
}
