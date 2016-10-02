pub trait Frontend {
    fn print_rom_dump(&mut self);
    fn run(&mut self);
}
