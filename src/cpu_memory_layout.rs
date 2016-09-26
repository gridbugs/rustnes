use ram::{NesRam, NES_RAM_NUM_BYTES};
use addressable::{CpuAddressable, Address, Result, Error};

const RAM_MIRROR_START: Address = 0;
const RAM_MIRROR_END: Address = 0x1fff;

const CARTRIDGE_START: Address = 0x6000;
const CARTRIDGE_END: Address = 0xffff;

pub struct NesCpuMemoryLayout<Cartridge: CpuAddressable> {
    ram: NesRam,
    cartridge: Cartridge,
}

fn resolve_mirrored_ram_address(address: Address) -> Address {
    address % (NES_RAM_NUM_BYTES as u16)
}

impl<Cartridge: CpuAddressable> NesCpuMemoryLayout<Cartridge> {
    pub fn new(cartridge: Cartridge) -> Self {
        NesCpuMemoryLayout {
            ram: NesRam::new(),
            cartridge: cartridge,
        }
    }
}

impl<Cartridge: CpuAddressable> CpuAddressable for NesCpuMemoryLayout<Cartridge> {
    fn read(&mut self, address: Address) -> Result<u8> {
        match address {
            RAM_MIRROR_START ... RAM_MIRROR_END => {
                let ram_address = resolve_mirrored_ram_address(address);
                self.ram.read(ram_address)
            },
            CARTRIDGE_START ... CARTRIDGE_END => {
                self.cartridge.read(address)
            },
            _ => {
                Err(Error::BusErrorRead(address))
            }
        }
    }
    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            RAM_MIRROR_START ... RAM_MIRROR_END => {
                let ram_address = resolve_mirrored_ram_address(address);
                self.ram.write(ram_address, data)
            },
            CARTRIDGE_START ... CARTRIDGE_END => {
                self.cartridge.write(address, data)
            },
            _ => {
                Err(Error::BusErrorWrite(address))
            }
        }
    }
}
