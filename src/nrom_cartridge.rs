use image::NesImage;
use cartridge;
use addressable;
use addressable::{PpuAddressable, Address};
use vram::NesVram;
use mirror::{Mirror, HorizontalMirror, VerticalMirror};
use image::VideoArrangement;

enum NromRom {
    OneBank(Vec<u8>),
    TwoBanks {
        lower: Vec<u8>,
        upper: Vec<u8>,
    },
}

pub struct NromCpuInterface {
    rom: NromRom,
    ram: Vec<u8>,
}

pub struct NromPpuInterface<M: Mirror> {
    internal_ram: NesVram,
    mirror: M,
    rom: Vec<u8>,
}

pub struct NromCartridgeWithMirror<M: Mirror> {
    cpu_interface: NromCpuInterface,
    ppu_interface: NromPpuInterface<M>,
}

pub enum NromCartridge {
    HorizontalMirroring(NromCartridgeWithMirror<HorizontalMirror>),
    VerticalMirroring(NromCartridgeWithMirror<VerticalMirror>),
}

impl NromCartridge {
    pub fn new(image: &NesImage) -> cartridge::Result<Self> {
        match image.header.video_arrangement {
            VideoArrangement::HorizontalMirroring => {
                Ok(NromCartridge::HorizontalMirroring(try!(NromCartridgeWithMirror::new(image, HorizontalMirror))))
            },
            VideoArrangement::VerticalMirroring => {
                Ok(NromCartridge::VerticalMirroring(try!(NromCartridgeWithMirror::new(image, VerticalMirror))))
            },
            _ => Err(cartridge::Error::InvalidNametableMirroring),
        }
    }
}

impl<M: Mirror> NromCartridgeWithMirror<M> {
    pub fn new(image: &NesImage, mirror: M) -> cartridge::Result<Self> {

        if image.header.mapper_number != cartridge::NROM {
            return Err(cartridge::Error::IncorrectMapper);
        }

        let rom = if image.header.prg_rom_size == 1 {
            let mut bank = vec![0; cartridge::ROM_BANK_SIZE];
            bank.copy_from_slice(&image.prg_rom[0..cartridge::ROM_BANK_SIZE]);
            NromRom::OneBank(bank)
        } else if image.header.prg_rom_size == 2 {
            let mut lower = vec![0; cartridge::ROM_BANK_SIZE];
            let mut upper = vec![0; cartridge::ROM_BANK_SIZE];
            lower.copy_from_slice(&image.prg_rom[0..cartridge::ROM_BANK_SIZE]);
            upper.copy_from_slice(
                &image.prg_rom[cartridge::ROM_BANK_SIZE..
                               cartridge::ROM_BANK_SIZE * 2]);
            NromRom::TwoBanks { lower: lower, upper: upper }
        } else {
            return Err(cartridge::Error::InvalidRomSize);
        };

        let ram = if let Some(ram_size) = image.header.prg_ram_size {
            vec![0; cartridge::RAM_BANK_SIZE * ram_size]
        } else {
            vec![0; cartridge::RAM_BANK_SIZE]
        };

        let chr_rom = if image.header.chr_rom_size == 1 {
            let mut bank = vec![0; cartridge::CHR_ROM_BANK_SIZE];
            bank.copy_from_slice(&image.chr_rom[0..cartridge::CHR_ROM_BANK_SIZE]);
            bank
        } else {
            return Err(cartridge::Error::InvalidChrRomSize);
        };


        Ok(NromCartridgeWithMirror {
            cpu_interface: NromCpuInterface {
                rom: rom,
                ram: ram,
            },
            ppu_interface: NromPpuInterface {
                internal_ram: NesVram::new(),
                mirror: mirror,
                rom: chr_rom,
            },
        })
    }
}

impl<M: Mirror> cartridge::Cartridge for NromCartridgeWithMirror<M> {
    type CpuInterface = NromCpuInterface;
    type PpuInterface = NromPpuInterface<M>;

    fn to_interfaces(self) -> (Self::CpuInterface, Self::PpuInterface) {
        (self.cpu_interface, self.ppu_interface)
    }
}

impl cartridge::CpuInterface for NromCpuInterface {
    fn ram_read(&mut self, address: Address) -> addressable::Result<u8> {
        Ok(self.ram[address as usize])
    }

    fn ram_write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.ram[address as usize] = data;
        Ok(())
    }

    fn lower_rom_read(&mut self, address: Address) -> addressable::Result<u8> {
        let bank = match &self.rom {
            &NromRom::OneBank(ref bank) => bank,
            &NromRom::TwoBanks { ref lower, upper: _ } => lower,
        };

        Ok(bank[address as usize])
    }

    fn lower_rom_write(&mut self, address: Address, _: u8) -> addressable::Result<()> {
        Err(addressable::Error::IllegalWrite(address))
    }

    fn upper_rom_read(&mut self, address: Address) -> addressable::Result<u8> {
        let bank = match &self.rom {
            &NromRom::OneBank(ref bank) => bank,
            &NromRom::TwoBanks { lower: _, ref upper } => upper,
        };

        Ok(bank[address as usize])
    }

    fn upper_rom_write(&mut self, address: Address, _: u8) -> addressable::Result<()> {
        Err(addressable::Error::IllegalWrite(address))
    }
}

impl<M: Mirror> cartridge::PpuInterface for NromPpuInterface<M> {
    fn pattern_table_read(&mut self, address: Address) -> addressable::Result<u8> {
        Ok(self.rom[address as usize])
    }
    fn pattern_table_write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        // despite this being rom, we implement writes to it
        self.rom[address as usize] = data;
        Ok(())
    }
    fn name_table_read(&mut self, address: Address) -> addressable::Result<u8> {
        self.internal_ram.read(M::mirror(address))
    }
    fn name_table_write(&mut self, address: Address, data: u8) -> addressable::Result<()> {
        self.internal_ram.write(M::mirror(address), data)
    }
}
