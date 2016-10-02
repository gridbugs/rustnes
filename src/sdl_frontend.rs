use sdl2;
use sdl2::{Sdl, EventPump};
use sdl2::render::{Texture, Renderer};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

use frontend::Frontend;
use cartridge;
use cartridge::Cartridge;
use nrom_cartridge::NromCartridge;
use nes::NesWithCartridge;
use image::NesImage;
use renderer;
use debug::NesDebug;

const SCALE: u32 = 2;
const WINDOW_WIDTH: u32 = renderer::DISPLAY_WIDTH as u32 * SCALE;
const WINDOW_HEIGHT: u32 = renderer::DISPLAY_HEIGHT as u32 * SCALE;

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

    fn frame(&mut self) {
        self.emulate_frame();
        self.render_texture();
    }

    fn print_state(&mut self) {
        println!("\nRAM{}", self.nes.dump_memory(0..0x7ff));
        println!("\nVRAM{}", self.nes.ppu_dump_memory(0x2000..0x2fff));
        println!("\nPalette{}", self.nes.ppu_dump_memory(0x3f00..0x3f1f));
        println!("\nCPU\n{}", self.nes.cpu);
        println!("\nPPU\n{}", self.nes.ppu);
    }
}

impl<'a, C: Cartridge> Frontend for SdlFrontend<'a, C> {
    fn print_rom_dump(&mut self) {
        println!("{}", self.nes.dump_rom());
    }

    fn run(&mut self) {

        self.init();

        for _ in 0..1000 {
            self.frame();
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
            0x06 => (0xff, 0, 0),
            _ => (0, 0xff, 0xff),
        }
    }
}

impl<'a> renderer::Frame for SdlFrame<'a> {
    fn set_pixel(&mut self, x: usize, y: usize, colour: u8) {
        let offset = y * self.pitch + x * 3;
        let (r, g, b) = Self::convert_colour(colour);
        self.buffer[offset + 0] = r;
        self.buffer[offset + 1] = g;
        self.buffer[offset + 2] = b;
    }
}
