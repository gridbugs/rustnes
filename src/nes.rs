use image::NesImage;
use cartridge;
use nrom_cartridge::NromCartridge;
use cpu_memory_layout::NesCpuMemoryLayout;
use ppu_memory_layout::NesPpuMemoryLayout;
use addressable;
use addressable::{Address, CpuAddressable};
use cpu::Cpu;
use ppu::Ppu;

// A Nes is CpuAddressable for debugging purposes
pub trait Nes: CpuAddressable {}

pub struct NesWithCartridge<C: cartridge::Cartridge> {
    cpu: Cpu<NesCpuMemoryLayout<C::CpuInterface>>,
    ppu: Ppu<NesPpuMemoryLayout<C::PpuInterface>>,
}

impl<C: cartridge::Cartridge> NesWithCartridge<C> {
    pub fn new(cartridge: C) -> Self {
        let (cpu_interface, ppu_interface) = cartridge.to_interfaces();

        let cpu_memory = NesCpuMemoryLayout::new(cpu_interface);
        let ppu_memory = NesPpuMemoryLayout::new(ppu_interface);

        NesWithCartridge {
            cpu: Cpu::new(cpu_memory),
            ppu: Ppu::new(ppu_memory),
        }
    }
}

impl<C: cartridge::Cartridge> CpuAddressable for NesWithCartridge<C> {
    fn read(&mut self, address: Address) -> addressable::Result<u8> {
        self.cpu.read(address)
    }

    fn write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.cpu.write(address, data)
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
                },
                NromCartridge::VerticalMirroring(cartridge) => {
                    Ok(Box::new(NesWithCartridge::new(cartridge)))
                },
            }
        },
        other => Err(cartridge::Error::UnknownMapper(other)),
    }
}
