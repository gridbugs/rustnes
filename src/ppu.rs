use addressable::{Addressable, Address, Result, Error};

const CONTROLLER: u16 = 0;
const MASK: u16 = 1;
const STATUS: u16 = 2;

const CONTROLLER_BASE_NAMETABLE_ADDRESS_MASK: u8 = mask!(2);
const CONTROLLER_VRAM_ADDRESS_INCREMENT: u8 = bit!(2);
const CONTROLLER_SPRITE_PATTERN_TABLE_8X8: u8 = bit!(3);
const CONTROLLER_BACKGROUND_PATTERN_TABLE: u8 = bit!(4);
const CONTROLLER_SPRITE_SIZE: u8 = bit!(5);
const CONTROLLER_PPU_MASTER_SLAVE_SELECT: u8 = bit!(6);
const CONTROLLER_VBLANK_NMI: u8 = bit!(7);

const MASK_GREYSCALE: u8 = bit!(0);
const MASK_BACKGROUND_LEFT: u8 = bit!(1);
const MASK_SPRITES_LEFT: u8 = bit!(2);
const MASK_BACKGROUND: u8 = bit!(3);
const MASK_SPRITES: u8 = bit!(4);
const MASK_EMPHASIZE_RED: u8 = bit!(5);
const MASK_EMPHASIZE_GREEN: u8 = bit!(6);
const MASK_EMPHASIZE_BLUE: u8 = bit!(7);

const STATUS_LAST_WRITE_MASK: u8 = mask!(5);
const STATUS_SPRITE_OVERFLOW: u8 = bit!(5);
const STATUS_SPRITE_0_HIT: u8 = bit!(6);
const STATUS_VBLANK: u8 = bit!(7);

pub struct PpuRegisterFile {
    controller: u8,
    mask: u8,
    status: u8,
}

impl PpuRegisterFile {
    fn new() -> Self {
        PpuRegisterFile {
            controller: 0,
            mask: 0,
            status: 0,
        }
    }
}

pub struct Ppu {
    pub registers: PpuRegisterFile,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { registers: PpuRegisterFile::new() }
    }

    pub fn vblank_start(&mut self) {
        self.registers.status |= STATUS_VBLANK;
    }

    pub fn vblank_end(&mut self) {

        self.registers.status &= !STATUS_VBLANK;
    }
}

impl Addressable for Ppu {
    fn read8(&mut self, address: Address) -> Result<u8> {
        match address {
            CONTROLLER => return Err(Error::IllegalRead(address)),
            MASK => return Err(Error::IllegalRead(address)),
            STATUS => {
                let value = self.registers.status;
                self.registers.status &= !STATUS_VBLANK;
                Ok(value)
            }
            _ => return Err(Error::UnimplementedRead(address)),
        }
    }

    fn write8(&mut self, address: Address, data: u8) -> Result<()> {

        self.registers.status |= data & STATUS_LAST_WRITE_MASK;

        match address {
            CONTROLLER => self.registers.controller = data,
            MASK => self.registers.mask = data,
            STATUS => return Err(Error::IllegalWrite(address)),
            _ => return Err(Error::UnimplementedWrite(address)),
        }
        Ok(())
    }
}
