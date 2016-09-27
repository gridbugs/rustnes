use image::NesImage;
use cartridge;
use nrom_cartridge::NromCartridge;
use cpu_memory_layout::NesCpuMemoryLayout;
use addressable;
use addressable::{Address, CpuAddressable, PpuAddressable};
use cpu::Cpu;

// A Nes is CpuAddressable for debugging purposes
pub trait Nes: CpuAddressable + PpuAddressable {}

pub struct NesWithCartridge<C: cartridge::Cartridge> {
    cpu: Cpu<NesCpuMemoryLayout<C>>,
}

impl<C: cartridge::Cartridge> NesWithCartridge<C> {
    pub fn new(cartridge: C) -> Self {

        NesWithCartridge { cpu: Cpu::new(NesCpuMemoryLayout::new(cartridge)) }
    }
}

impl<C: cartridge::Cartridge> CpuAddressable for NesWithCartridge<C> {
    fn read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.cpu.read8(address)
    }

    fn write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.cpu.write8(address, data)
    }
}

impl<C: cartridge::Cartridge> PpuAddressable for NesWithCartridge<C> {
    fn ppu_read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.cpu.ppu_read8(address)
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.cpu.ppu_write8(address, data)
    }
}

impl<C: cartridge::Cartridge> Nes for NesWithCartridge<C> {}

// Creates a new nes emulator instance. This uses a trait object to prevent
// the top-level nes type needing to be paramerized by a cartridge type.
// Using a trait object here, as opposed to deeper in the nes emulator, allows
// all the internal code to be statically polymorphic, allowing for greater
// compiler optimizations.
pub fn make_nes(image: &NesImage) -> cartridge::Result<Box<Nes>> {
    match image.header.mapper_number {
        cartridge::NROM => {
            match try!(NromCartridge::new(image)) {
                NromCartridge::HorizontalMirroring(cartridge) => {
                    Ok(Box::new(NesWithCartridge::new(cartridge)))
                }
                NromCartridge::VerticalMirroring(cartridge) => {
                    Ok(Box::new(NesWithCartridge::new(cartridge)))
                }
            }
        }
        other => Err(cartridge::Error::UnknownMapper(other)),
    }
}
