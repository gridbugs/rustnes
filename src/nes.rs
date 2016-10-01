use image::NesImage;
use cartridge;
use nrom_cartridge::NromCartridge;
use memory_layout::MemoryLayout;
use addressable;
use addressable::{Address, Addressable, PpuAddressable};
use cpu;
use cpu::{Cpu, RegisterFile};
use ppu::Ppu;
use io::Io;
use ram::NesRam;
use vram::NesVram;
use palette::Palette;

pub trait Nes: Addressable + PpuAddressable {
    fn init(&mut self) -> cpu::Result<()>;
    fn cpu_registers(&self) -> &RegisterFile;
    fn emulate_frame(&mut self) -> cpu::Result<()>;
    fn emulate_loop(&mut self) -> cpu::Result<()>;
}

pub struct NesWithCartridge<C: cartridge::Cartridge> {
    cartridge: C,
    cpu: Cpu,
    ppu: Ppu,
    io: Io,
    ram: NesRam,
    vram: NesVram,
    palette: Palette,
}

impl<C: cartridge::Cartridge> NesWithCartridge<C> {
    pub fn new(cartridge: C) -> Self {
        NesWithCartridge {
            cartridge: cartridge,
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            io: Io::new(),
            ram: NesRam::new(),
            vram: NesVram::new(),
            palette: Palette::new(),
        }
    }

    pub fn memory_layout(&mut self) -> MemoryLayout<C> {
        MemoryLayout::new(&mut self.cartridge, &mut self.ppu, &mut self.io, &mut self.ram, &mut self.vram, &mut self.palette)
    }

    fn vblank_interval(&mut self) -> cpu::Result<()> {
        let mut interrupts = self.cpu.interrupts;
        interrupts = self.ppu.vblank_start(interrupts);
        self.cpu.interrupts = interrupts;

        try!(self.emulate_cpu(2000));

        Ok(())
    }

    fn render_interval(&mut self) -> cpu::Result<()> {
        let mut interrupts = self.cpu.interrupts;
        interrupts = self.ppu.vblank_end(interrupts);
        self.cpu.interrupts = interrupts;

        try!(self.emulate_cpu(2000));

        Ok(())
    }

    fn emulate_cpu(&mut self, num_instructions: usize) -> cpu::Result<()> {
        let mut cpu = self.cpu;

        cpu = try!(emulate_cpu(cpu, &mut self.memory_layout(), num_instructions));

        self.cpu = cpu;
        Ok(())
    }
}

fn emulate_cpu<A: cartridge::Cartridge>(mut cpu: Cpu,
                               memory: &mut MemoryLayout<A>,
                               num_instructions: usize)
                               -> cpu::Result<(Cpu)> {

    for _ in 0..num_instructions {

        cpu = try!(emulate_cpu_instruction(cpu, memory));
    }

    Ok(cpu)
}

fn emulate_cpu_instruction<A: cartridge::Cartridge>(mut cpu: Cpu,
                                           memory: &mut MemoryLayout<A>)
                                           -> cpu::Result<(Cpu)> {
    try!(cpu.tick(memory));

    Ok(cpu)
}

impl<C: cartridge::Cartridge> Addressable for NesWithCartridge<C> {
    fn read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.memory_layout().read8(address)
    }

    fn write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.memory_layout().write8(address, data)
    }
}

impl<C: cartridge::Cartridge> PpuAddressable for NesWithCartridge<C> {
    fn ppu_read8(&mut self, address: Address) -> addressable::Result<u8> {
        self.memory_layout().ppu_memory_layout().ppu_read8(address)
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.memory_layout().ppu_memory_layout().ppu_write8(address, data)
    }
}

impl<C: cartridge::Cartridge> Nes for NesWithCartridge<C> {
    fn init(&mut self) -> cpu::Result<()> {
        let mut cpu = self.cpu;
        try!(cpu.init(&mut self.memory_layout()));
        self.cpu = cpu;

        Ok(())
    }

    fn cpu_registers(&self) -> &RegisterFile {
        &self.cpu.registers
    }

    fn emulate_frame(&mut self) -> cpu::Result<()> {
        try!(self.vblank_interval());
        try!(self.render_interval());

        Ok(())
    }

    fn emulate_loop(&mut self) -> cpu::Result<()> {
        for _ in 0..1000 {
            try!(self.emulate_frame());
            //println!("#############################################");
        }

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
