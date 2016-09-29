use image::NesImage;
use cartridge;
use nrom_cartridge::NromCartridge;
use cpu_memory_layout::NesCpuMemoryLayout;
use ppu_memory_layout::NesPpuMemoryLayout;
use cpu_memory_layout_change::{MemoryWrite, NesCpuMemoryLayoutBuffer};
use addressable;
use addressable::{Address, Addressable, PpuAddressable, CartridgeAddressable};
use cpu;
use cpu::{Cpu, RegisterFile};
use ppu::Ppu;
use ram::NesRam;
use vram::NesVram;

pub trait Nes: Addressable + PpuAddressable {
    fn init(&mut self) -> cpu::Result<()>;
    fn cpu_registers(&self) -> &RegisterFile;
    fn cpu_tick(&mut self) -> cpu::Result<()>;
}

pub struct NesWithCartridge<C: CartridgeAddressable> {
    cartridge: C,
    cpu: Cpu,
    ppu: Ppu,
    ram: NesRam,
    vram: NesVram,
    write_buffer: Vec<MemoryWrite>,
    read_buffer: Vec<Address>,
}

impl<C: CartridgeAddressable> NesWithCartridge<C> {
    pub fn new(cartridge: C) -> Self {
        NesWithCartridge {
            cartridge: cartridge,
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            ram: NesRam::new(),
            vram: NesVram::new(),
            write_buffer: Vec::new(),
            read_buffer: Vec::new(),
        }
    }

    pub fn cpu_memory_layout(&mut self) -> NesCpuMemoryLayout<C> {
        NesCpuMemoryLayout::new(&mut self.cartridge, &mut self.ppu.registers, &mut self.ram)
    }

    pub fn ppu_memory_layout(&mut self) -> NesPpuMemoryLayout<C> {
        NesPpuMemoryLayout::new(&mut self.cartridge)
    }

    pub fn cpu_memory_layout_buffer(&mut self) -> NesCpuMemoryLayoutBuffer<C> {
        NesCpuMemoryLayoutBuffer::new(NesCpuMemoryLayout::new(&mut self.cartridge,
                                                              &mut self.ppu.registers,
                                                              &mut self.ram),
                                      &mut self.write_buffer,
                                      &mut self.read_buffer)
    }
}

impl<C: CartridgeAddressable> Addressable for NesWithCartridge<C> {
    fn read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.cpu_memory_layout().read8(address)
    }

    fn write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.cpu_memory_layout().write8(address, data)
    }
}

impl<C: CartridgeAddressable> PpuAddressable for NesWithCartridge<C> {
    fn ppu_read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.ppu_memory_layout().ppu_read8(address)
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.ppu_memory_layout().ppu_write8(address, data)
    }
}

impl<C: CartridgeAddressable> Nes for NesWithCartridge<C> {
    fn init(&mut self) -> cpu::Result<()> {
        let mut cpu = self.cpu;
        try!(cpu.init(&mut self.cpu_memory_layout()));
        self.cpu = cpu;

        Ok(())
    }

    fn cpu_registers(&self) -> &RegisterFile {
        &self.cpu.registers
    }

    fn cpu_tick(&mut self) -> cpu::Result<()> {
        let mut cpu = self.cpu;

        {
            let mut buffer = self.cpu_memory_layout_buffer();
            try!(cpu.tick(&mut buffer));
            try!(buffer.apply().map_err(cpu::Error::MemoryError));
        }

        self.cpu = cpu;

        Ok(())
    }
}

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
