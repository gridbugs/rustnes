use addressable::{PpuAddressable, Address, AddressDiff, Result, Error};
use cartridge::NAME_TABLE_START;

const CARTRIDGE_START: Address = 0x0000;
const CARTRIDGE_END: Address = 0x2fff;
const NAME_TABLE_MIRROR_START: Address = 0x3000;
const NAME_TABLE_MIRROR_END: Address = 0x3eff;
const PALETTE_START: Address = 0x3f00;
const PALETTE_END: Address = 0x3f1f;
const PALETTE_MIRROR_START: Address = 0x3f20;
const PALETTE_MIRROR_END: Address = 0x3fff;

const NAME_TABLE_MIRROR_OFFSET: AddressDiff = NAME_TABLE_MIRROR_START - NAME_TABLE_START;
const PALETTE_SIZE: AddressDiff = PALETTE_END - PALETTE_START + 1;

pub struct NesPpuMemoryLayout<'a, C: 'a + PpuAddressable> {
    cartridge: &'a mut C,
}

impl<'a, C: 'a + PpuAddressable> NesPpuMemoryLayout<'a, C> {
    pub fn new(cartridge: &'a mut C) -> Self {
        NesPpuMemoryLayout { cartridge: cartridge }
    }
}

impl<'a, C: 'a + PpuAddressable> PpuAddressable for NesPpuMemoryLayout<'a, C> {
    fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        match address {
            CARTRIDGE_START...CARTRIDGE_END => self.cartridge.ppu_read8(address),
            NAME_TABLE_MIRROR_START...NAME_TABLE_MIRROR_END => {
                self.cartridge.ppu_read8(address - NAME_TABLE_MIRROR_OFFSET)
            }
            PALETTE_START...PALETTE_END => unimplemented!(),
            PALETTE_MIRROR_START...PALETTE_MIRROR_END => unimplemented!(),
            _ => Err(Error::BusErrorRead(address)),
        }
    }

    fn ppu_write8(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            CARTRIDGE_START...CARTRIDGE_END => self.cartridge.ppu_write8(address, data),
            NAME_TABLE_MIRROR_START...NAME_TABLE_MIRROR_END => {
                self.cartridge.ppu_write8(address - NAME_TABLE_MIRROR_OFFSET, data)
            }
            PALETTE_START...PALETTE_END => unimplemented!(),
            PALETTE_MIRROR_START...PALETTE_MIRROR_END => unimplemented!(),
            _ => Err(Error::BusErrorWrite(address)),
        }
    }
}
