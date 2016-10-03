use std::thread;
use std::time::Duration;

use sdl2;
use sdl2::{Sdl, EventPump};
use sdl2::render::{Texture, Renderer};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use frontend::Frontend;
use cartridge;
use cartridge::Cartridge;
use nrom_cartridge::NromCartridge;
use nes::NesWithCartridge;
use image::NesImage;
use renderer;
use debug::NesDebug;
use ppu;
use io;

const SCALE: u32 = 2;
const WINDOW_WIDTH: u32 = ppu::DISPLAY_WIDTH as u32 * SCALE;
const WINDOW_HEIGHT: u32 = ppu::DISPLAY_HEIGHT as u32 * SCALE;

enum MetaControl {
    Quit,
}

pub struct SdlFrontend<'a, C: Cartridge> {
    nes: NesWithCartridge<C>,
    sdl: Sdl,
    events: EventPump,
    renderer: Renderer<'a>,
    texture: Texture,
}

struct SdlFrame<'a> {
    buffer: &'a mut [u8],
    pitch: usize,
}

impl<'a> SdlFrame<'a> {
    fn new(buffer: &'a mut [u8], pitch: usize) -> Self {
        SdlFrame {
            buffer: buffer,
            pitch: pitch,
        }
    }
}

impl<'a, C: Cartridge> SdlFrontend<'a, C> {
    pub fn new(cartridge: C) -> Self {
        let sdl = sdl2::init().expect("SDL2 initialization failed");
        let window = sdl.video().unwrap()
            .window("NES", WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()
            .expect("Failed to create window");
        let events = sdl.event_pump().expect("Failed to initialise events");
        let renderer = window.renderer().build()
            .expect("Failed to initialise renderer");

        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, 256, 240)
            .expect("Failed to initialise texture");

        SdlFrontend {
            nes: NesWithCartridge::new(cartridge),
            sdl: sdl,
            events: events,
            renderer: renderer,
            texture: texture,
        }
    }

    fn init(&mut self) {
        self.nes.init().expect("Failed to initialise nes");
    }

    fn render_texture(&mut self) {
        self.renderer.clear();
        self.renderer.copy(&self.texture, None, Some(Rect::new(0, 0, 512, 480)));
        self.renderer.present();
    }

    fn emulate_frame(&mut self) {
        let nes = &mut self.nes;
        self.texture.with_lock(None, |buffer, pitch| {
            let mut frame = SdlFrame::new(buffer, pitch);
            nes.emulate_frame(&mut frame).expect("Emulation failed");
        }).unwrap();
    }

    fn frame(&mut self) -> Option<MetaControl> {
        let meta = self.get_input();

        self.emulate_frame();
        self.render_texture();
        thread::sleep(Duration::from_millis(10));

        meta
    }

    fn print_state(&mut self) {
        println!("\nRAM{}", self.nes.dump_memory(0..0x7ff));
        println!("\nVRAM{}", self.nes.ppu_dump_memory(0x2000..0x2fff));
        println!("\nPalette{}", self.nes.ppu_dump_memory(0x3f00..0x3f1f));
        println!("\nCPU\n{}", self.nes.cpu);
        println!("\nPPU\n{}", self.nes.ppu);
    }

    fn get_input(&mut self) -> Option<MetaControl> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Some(MetaControl::Quit);
                }
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_START);
                }
                Event::KeyDown { keycode: Some(Keycode::RShift), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_SELECT);
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_A);
                }
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_B);
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_LEFT);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_RIGHT);
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_UP);
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.nes.io.joy1_press(io::BUTTON_DOWN);
                }
                _ => {}
            }
        }

        None
    }
}

impl<'a, C: Cartridge> Frontend for SdlFrontend<'a, C> {
    fn print_rom_dump(&mut self) {
        println!("{}", self.nes.dump_rom());
    }

    fn run(&mut self) {

        self.init();

        loop {
            if let Some(MetaControl::Quit) = self.frame() {
                break;
            }
        }

        self.print_state();
    }
}


pub fn init(image: &NesImage) -> cartridge::Result<Box<Frontend>> {
    match image.header.mapper_number {
        cartridge::NROM => {
            match try!(NromCartridge::new(image)) {
                NromCartridge::HorizontalMirroring(cartridge) => {
                    Ok(Box::new(SdlFrontend::new(cartridge)))
                }
                NromCartridge::VerticalMirroring(cartridge) => {
                    Ok(Box::new(SdlFrontend::new(cartridge)))
                }
            }
        }
        other => Err(cartridge::Error::UnknownMapper(other)),
    }
}

impl<'a> SdlFrame<'a> {
    fn convert_colour(nes_colour: u8) -> (u8, u8, u8) {
        match nes_colour {
            0x00 => (124, 124, 124),
            0x01 => (0, 0, 252),
            0x02 => (0, 0, 118),
            0x03 => (68, 40, 188),
            0x04 => (140, 0, 32),
            0x05 => (168, 16, 0),
            0x06 => (168, 0, 16),
            0x07 => (136, 20, 0),
            0x08 => (80, 48, 0),
            0x09 => (0, 120, 0),
            0x0a => (0, 104, 0),
            0x0b => (0, 88, 0),
            0x0c => (0, 64, 88),
            0x0d => (0, 0, 0),
            0x0e => (0, 0, 0),
            0x0f => (0, 0, 0),
            0x10 => (188, 188, 188),
            0x11 => (0, 120, 248),
            0x12 => (0, 88, 248),
            0x13 => (104, 68, 252),
            0x14 => (216, 0, 204),
            0x15 => (228, 0, 88),
            0x16 => (248, 56, 0),
            0x17 => (228, 92, 16),
            0x18 => (172, 124, 0),
            0x19 => (0, 184, 0),
            0x1a => (0, 168, 0),
            0x1b => (0, 168, 68),
            0x1c => (0, 136, 136),
            0x1d => (0, 0, 0),
            0x1e => (0, 0, 0),
            0x1f => (0, 0, 0),
            0x20 => (248, 248, 248),
            0x21 => (60, 188, 252),
            0x22 => (104, 136, 252),
            0x23 => (152, 120, 248),
            0x24 => (248, 120, 248),
            0x25 => (248, 88, 152),
            0x26 => (248, 120, 88),
            0x27 => (252, 160, 68),
            0x28 => (248, 184, 0),
            0x29 => (184, 248, 24),
            0x2a => (88, 216, 84),
            0x2b => (88, 248, 152),
            0x2c => (0, 232, 216),
            0x2d => (120, 120, 120),
            0x2e => (0, 0, 0),
            0x2f => (0, 0, 0),
            0x30 => (252, 252, 252),
            0x31 => (164, 228, 252),
            0x32 => (184, 184, 248),
            0x33 => (216, 184, 248),
            0x34 => (248, 184, 248),
            0x35 => (248, 164, 192),
            0x36 => (240, 208, 176),
            0x37 => (252, 224, 168),
            0x38 => (248, 216, 120),
            0x39 => (216, 248, 120),
            0x3a => (184, 248, 184),
            0x3b => (184, 248, 216),
            0x3c => (0, 252, 252),
            0x3d => (216, 216, 216),
            0x3e => (0, 0, 0),
            0x3f => (0, 0, 0),
            _ => panic!("{:2x}", nes_colour),
        }
    }
}

impl<'a> renderer::Frame for SdlFrame<'a> {
    fn set_pixel(&mut self, x: usize, y: usize, colour: u8) {
        let offset = y * self.pitch + x * 3;
        if offset + 2 < self.buffer.len() {
            let (r, g, b) = Self::convert_colour(colour);
            self.buffer[offset + 0] = r;
            self.buffer[offset + 1] = g;
            self.buffer[offset + 2] = b;
        }
    }
}
