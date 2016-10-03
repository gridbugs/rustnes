use cartridge;
use memory_layout::MemoryLayout;
use addressable;
use addressable::{Address, Addressable, PpuAddressable};
use cpu;
use cpu::Cpu;
use ppu::Ppu;
use io::Io;
use ram::NesRam;
use vram::NesVram;
use palette::Palette;
use renderer::Frame;
use ppu_memory_layout::PpuMemoryLayout;

pub struct NesWithCartridge<C: cartridge::Cartridge> {
    cartridge: C,
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub io: Io,
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

    pub fn init(&mut self) -> cpu::Result<()> {
        let mut cpu = self.cpu;
        try!(cpu.init(&mut self.memory_layout()));
        self.cpu = cpu;

        Ok(())
    }

    pub fn emulate_frame<F: Frame>(&mut self, frame: &mut F) -> cpu::Result<()> {
        try!(self.vblank_interval());
        try!(self.render_interval(frame));

        Ok(())
    }

    fn memory_layout(&mut self) -> MemoryLayout<C> {
        MemoryLayout::new(&mut self.cartridge,
                          &mut self.ppu,
                          &mut self.io,
                          &mut self.ram,
                          &mut self.vram,
                          &mut self.palette)
    }

    fn vblank_interval(&mut self) -> cpu::Result<()> {
        let mut interrupts = self.cpu.interrupts;
        interrupts = self.ppu.vblank_start(interrupts);
        self.cpu.interrupts = interrupts;

        try!(self.emulate_cpu(2000));

        Ok(())
    }

    fn render_interval<F: Frame>(&mut self, frame: &mut F) -> cpu::Result<()> {
        let mut interrupts = self.cpu.interrupts;
        interrupts = self.ppu.vblank_end(interrupts);
        self.cpu.interrupts = interrupts;

        {
            let mut ppu_memory = PpuMemoryLayout::new(&mut self.cartridge, &mut self.vram, &mut self.palette);

            try!(self.ppu.render(frame, &mut ppu_memory).map_err(cpu::Error::MemoryError));
        }

        try!(self.emulate_cpu(2000));

        self.ppu.render_end();

        Ok(())
    }

    fn emulate_cpu(&mut self, num_instructions: usize) -> cpu::Result<()> {
        let mut cpu = self.cpu;

        for _ in 0..num_instructions {
            try!(cpu.tick(&mut self.memory_layout()));
        }

        self.cpu = cpu;
        Ok(())
    }
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
