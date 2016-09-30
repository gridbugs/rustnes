use ram::NesRam;
use addressable::{Addressable, Address, AddressDiff, Result, Error};
use ppu::Ppu;

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

pub struct NesCpuMemoryLayout<'a, C: 'a + Addressable> {
    cartridge: &'a mut C,
    ppu: &'a mut Ppu,
    ram: &'a mut NesRam,
}

impl<'a, C: 'a + Addressable> NesCpuMemoryLayout<'a, C> {
    pub fn new(cartridge: &'a mut C,
               ppu: &'a mut Ppu,
               ram: &'a mut NesRam)
               -> Self {

        NesCpuMemoryLayout {
            cartridge: cartridge,
            ppu: ppu,
            ram: ram,
        }
    }
}

impl<'a, C: 'a + Addressable> Addressable for NesCpuMemoryLayout<'a, C> {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.read8(address % RAM_SIZE),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                self.ppu.read8((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE)
            }
            CARTRIDGE_START...CARTRIDGE_END => self.cartridge.read8(address - CARTRIDGE_START),
            _ => Err(Error::UnimplementedRead(address)),
        }
    }
    fn write8(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.write8(address % RAM_SIZE, data),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                self.ppu.write8((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE, data)
            }
            CARTRIDGE_START...CARTRIDGE_END => {
                self.cartridge.write8(address - CARTRIDGE_START, data)
            }
            _ => Err(Error::UnimplementedWrite(address)),
        }
    }
}
