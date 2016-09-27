use ram::{NesRam, NES_RAM_NUM_BYTES};
use addressable::{CpuAddressable, PpuAddressable, Address, Result, Error};
use cartridge::Cartridge;
use ppu::Ppu;
use ppu_memory_layout::NesPpuMemoryLayout;

const RAM_MIRROR_START: Address = 0;
const RAM_MIRROR_END: Address = 0x1fff;

const CARTRIDGE_START: Address = 0x6000;
const CARTRIDGE_END: Address = 0xffff;

pub struct NesCpuMemoryLayout<C: Cartridge> {
    ram: NesRam,
    cartridge: C::CpuInterface,
    ppu: Ppu<NesPpuMemoryLayout<C::PpuInterface>>,
}

fn resolve_mirrored_ram_address(address: Address) -> Address {
    address % (NES_RAM_NUM_BYTES as u16)
}

impl<C: Cartridge> NesCpuMemoryLayout<C> {
    pub fn new(cartridge: C) -> Self {

        let (cpu_interface, ppu_interface) = cartridge.to_interfaces();

        NesCpuMemoryLayout {
            ram: NesRam::new(),
            cartridge: cpu_interface,
            ppu: Ppu::new(NesPpuMemoryLayout::new(ppu_interface)),
        }
    }
}

impl<C: Cartridge> CpuAddressable for NesCpuMemoryLayout<C> {
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

impl<C: Cartridge> PpuAddressable for NesCpuMemoryLayout<C> {
    fn ppu_read(&mut self, address: Address) -> Result<u8> {
        self.ppu.ppu_read(address)
    }

    fn ppu_write(&mut self, address: Address, data: u8) -> Result<()> {
        self.ppu.ppu_write(address, data)
    }
}
