use image::NesImage;
use cartridge;
use nrom_cartridge::NromCartridge;
use cpu_memory_layout::NesCpuMemoryLayout;
use addressable;
use addressable::{Address, Addressable};
use mos6502::Mos6502;

// A Nes is Addressable for debugging purposes
pub trait Nes: Addressable {}

pub struct NesWithCartridge<Memory: Addressable> {
    cpu: Mos6502<Memory>,
}

impl<Memory: Addressable> NesWithCartridge<Memory> {
    pub fn new(memory: Memory) -> Self {
        NesWithCartridge {
            cpu: Mos6502::new(memory),
        }
    }
}

impl<Memory: Addressable> Addressable for NesWithCartridge<Memory> {
    fn read(&mut self, address: Address) -> addressable::Result<u8> {
        self.cpu.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.cpu.write(address, data)
    }
}

impl<Memory: Addressable> Nes for NesWithCartridge<Memory> {}

fn make_nes_from_cartridge<Cartridge: 'static + Addressable>(cartridge: Cartridge) -> Box<Nes> {
    let memory = NesCpuMemoryLayout::new(cartridge);
    Box::new(NesWithCartridge::new(memory))
}

// Creates a new nes emulator instance. This uses a trait object to prevent
// the top-level nes type needing to be paramerized by a cartridge type.
// Using a trait object here, as opposed to deeper in the nes emulator, allows
// all the internal code to be statically polymorphic, allowing for greater
// compiler optimizations.
pub fn make_nes(image: &NesImage) -> cartridge::Result<Box<Nes>> {
    match image.header.mapper_number {
        cartridge::NROM => {
            let cartridge = try!(NromCartridge::new(image));
            Ok(make_nes_from_cartridge(cartridge))
        },
        other => Err(cartridge::Error::UnknownMapper(other)),
    }
}
