use ram::NesRam;
use vram::NesVram;
use cartridge::Cartridge;
use addressable::{Addressable, Address, AddressDiff, Result, Error};
use ppu::Ppu;
use ppu_memory_layout::PpuMemoryLayout;
use io::Io;
use palette::Palette;

const RAM_START: Address = 0x0000;
const RAM_END: Address = 0x07ff;
const RAM_MIRROR_END: Address = 0x1fff;
const RAM_SIZE: AddressDiff = RAM_END - RAM_START + 1;

const PPU_REGISTER_START: Address = 0x2000;
const PPU_REGISTER_END: Address = 0x2007;
const PPU_REGISTER_MIRROR_START: Address = 0x2008;
const PPU_REGISTER_MIRROR_END: Address = 0x3fff;
const PPU_REGISTER_SIZE: AddressDiff = PPU_REGISTER_END - PPU_REGISTER_START + 1;

const IO_PORTS_START: Address = 0x4000;
const IO_PORTS_END: Address = 0x401f;

const EXPANSION_ROM_START: Address = 0x4020;
const EXPANSION_ROM_END: Address = 0x5fff;

const CARTRIDGE_START: Address = 0x6000;
const CARTRIDGE_END: Address = 0xffff;

const IO_REGISTER_START: Address = 0x4000;
const IO_REGISTER_END: Address = 0x401f;

const PPU_OAM_DMA: Address = 0x4014;

pub struct MemoryLayout<'a, C: 'a + Cartridge> {
    cartridge: &'a mut C,
    ppu: &'a mut Ppu,
    io: &'a mut Io,
    ram: &'a mut NesRam,
    vram: &'a mut NesVram,
    palette: &'a mut Palette,
}

impl<'a, C: 'a + Cartridge> MemoryLayout<'a, C> {
    pub fn new(cartridge: &'a mut C,
               ppu: &'a mut Ppu,
               io: &'a mut Io,
               ram: &'a mut NesRam,
               vram: &'a mut NesVram,
               palette: &'a mut Palette)
               -> Self {

        MemoryLayout {
            cartridge: cartridge,
            ppu: ppu,
            io: io,
            ram: ram,
            vram: vram,
            palette: palette,
        }
    }

    fn ppu_oam_dma(&mut self, address: u8) -> Result<()> {
        println!("DMA from page {:02x}", address);
        Ok(())
    }

    pub fn ppu_memory_layout(&mut self) -> PpuMemoryLayout<C> {
        PpuMemoryLayout::new(self.cartridge, self.vram, self.palette)
    }
}

impl<'a, C: 'a + Cartridge> Addressable for MemoryLayout<'a, C> {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.read8(address % RAM_SIZE),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                let ppu_memory = PpuMemoryLayout::new(self.cartridge, self.vram, self.palette);
                self.ppu.read8((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE, ppu_memory)
            }
            IO_REGISTER_START...IO_REGISTER_END => {
                self.io.read8(address - IO_REGISTER_START)
            }
            CARTRIDGE_START...CARTRIDGE_END => self.cartridge.read8(address - CARTRIDGE_START),
            _ => Err(Error::UnimplementedRead(address)),
        }
    }
    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        if address == PPU_OAM_DMA {
            return self.ppu_oam_dma(data);
        }
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.write8(address % RAM_SIZE, data),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                let ppu_memory = PpuMemoryLayout::new(self.cartridge, self.vram, self.palette);
                self.ppu.write8((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE, data, ppu_memory)
            }
            IO_REGISTER_START...IO_REGISTER_END => {
                self.io.write8(address - IO_REGISTER_START, data)
            }
            CARTRIDGE_START...CARTRIDGE_END => {
                self.cartridge.write8(address - CARTRIDGE_START, data)
            }
            _ => Err(Error::UnimplementedWrite(address)),
        }
    }
}
