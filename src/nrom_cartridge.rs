use image::NesImage;
use cartridge;
use addressable;
use addressable::Address;

#[derive(Debug)]
enum NromRom {
    OneBank(Vec<u8>),
    TwoBanks {
        lower: Vec<u8>,
        upper: Vec<u8>,
    },
}

#[derive(Debug)]
pub struct NromCartridge {
    rom: NromRom,
    ram: Vec<u8>,
}

impl NromCartridge {
    pub fn new(image: &NesImage) -> cartridge::Result<Self> {

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

        Ok(NromCartridge {
            rom: rom,
            ram: ram,
        })
    }
}

impl cartridge::AddressableCartridge for NromCartridge {
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
