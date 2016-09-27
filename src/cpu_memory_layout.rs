use ram::NesRam;
use addressable::{CpuAddressable, PpuAddressable, Address, AddressDiff, Result, Error};
use cartridge::Cartridge;
use ppu::Ppu;
use ppu_memory_layout::NesPpuMemoryLayout;
use io_ports::NesIoPorts;
use expansion::NesExpansionRom;

const RAM_START: Address = 0x0000;
const RAM_END: Address = 0x0800;
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

pub struct NesCpuMemoryLayout<C: Cartridge> {
    ram: NesRam,
    cartridge: C::CpuInterface,
    ppu: Ppu<NesPpuMemoryLayout<C::PpuInterface>>,
    io_ports: NesIoPorts,
    expansion: NesExpansionRom,
}

impl<C: Cartridge> NesCpuMemoryLayout<C> {
    pub fn new(cartridge: C) -> Self {

        let (cpu_interface, ppu_interface) = cartridge.to_interfaces();

        NesCpuMemoryLayout {
            ram: NesRam::new(),
            cartridge: cpu_interface,
            ppu: Ppu::new(NesPpuMemoryLayout::new(ppu_interface)),
            io_ports: NesIoPorts::new(),
            expansion: NesExpansionRom::new(),
        }
    }
}

impl<C: Cartridge> CpuAddressable for NesCpuMemoryLayout<C> {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.read8(address % RAM_SIZE),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                self.ppu.read8((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE)
            }
            IO_PORTS_START...IO_PORTS_END => self.io_ports.read8(address - IO_PORTS_START),
            EXPANSION_ROM_START...EXPANSION_ROM_END => {
                self.expansion.read8(address - EXPANSION_ROM_START)
            }
            CARTRIDGE_START...CARTRIDGE_END => self.cartridge.read8(address - CARTRIDGE_START),
            _ => Err(Error::BusErrorRead(address)),
        }
    }
    fn write(&mut self, address: Address, data: u8) -> Result<()> {
        match address {
            RAM_START...RAM_MIRROR_END => self.ram.write(address % RAM_SIZE, data),
            PPU_REGISTER_START...PPU_REGISTER_MIRROR_END => {
                self.ppu.write((address - PPU_REGISTER_START) % PPU_REGISTER_SIZE, data)
            }
            IO_PORTS_START...IO_PORTS_END => self.io_ports.write(address - IO_PORTS_START, data),
            EXPANSION_ROM_START...EXPANSION_ROM_END => {
                self.expansion.write(address - EXPANSION_ROM_START, data)
            }
            CARTRIDGE_START...CARTRIDGE_END => {
                self.cartridge.write(address - CARTRIDGE_START, data)
            }

            _ => Err(Error::BusErrorWrite(address)),
        }
    }
}

impl<C: Cartridge> PpuAddressable for NesCpuMemoryLayout<C> {
    fn ppu_read8(&mut self, address: Address) -> Result<u8> {
        self.ppu.ppu_read8(address)
    }

    fn ppu_write(&mut self, address: Address, data: u8) -> Result<()> {
        self.ppu.ppu_write(address, data)
    }
}
