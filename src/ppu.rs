use addressable::{PpuAddressable, Address, Result, Error};

const CONTROLLER: Address = 0;
const MASK: Address = 1;
const STATUS: Address = 2;
const OAM_ADDRESS: Address = 3;
const OAM_DATA: Address = 4;
const SCROLL: Address = 5;
const ADDRESS: Address = 6;
const DATA: Address = 7;

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

enum ScrollAxis { X, Y }
enum AddressPhase { LOW, HIGH }

pub struct PpuRegisterFile {
    controller: u8,
    mask: u8,
    status: u8,
    oam_address: u8,
    scroll: u8,
    address: u8,
    data: u8,
}

impl PpuRegisterFile {
    fn new() -> Self {
        PpuRegisterFile {
            controller: 0,
            mask: 0,
            status: 0,
            oam_address: 0,
            scroll: 0,
            address: 0,
            data: 0,
        }
    }
}

pub struct Ppu {
    pub registers: PpuRegisterFile,
    scroll_axis: ScrollAxis,
    scroll_x: u8,
    scroll_y: u8,
    address_phase: AddressPhase,
    address: Address,
    oam: Vec<u8>,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            registers: PpuRegisterFile::new(),
            scroll_axis: ScrollAxis::X,
            scroll_x: 0,
            scroll_y: 0,
            address_phase: AddressPhase::HIGH,
            address: 0,
            oam: vec![0; 256],
        }
    }

    pub fn vblank_start(&mut self) {
        self.registers.status |= STATUS_VBLANK;
    }

    pub fn vblank_end(&mut self) {
        self.registers.status &= !STATUS_VBLANK;
    }

    fn increment_address(&mut self) {
        if self.registers.controller & CONTROLLER_VRAM_ADDRESS_INCREMENT != 0 {
            self.address = self.address.wrapping_add(32);
        } else {
            self.address = self.address.wrapping_add(1);
        }
    }

    pub fn read8<Memory: PpuAddressable>(&mut self, address: Address, mut memory: Memory) -> Result<u8> {
        let data = match address {
            CONTROLLER => return Err(Error::IllegalRead(address)),
            MASK => return Err(Error::IllegalRead(address)),
            STATUS => {
                let value = self.registers.status;
                self.registers.status &= !STATUS_VBLANK;
                value
            }
            OAM_ADDRESS => return Err(Error::IllegalRead(address)),
            OAM_DATA => self.oam[self.registers.oam_address as usize],
            SCROLL => return Err(Error::IllegalRead(address)),
            ADDRESS => return Err(Error::IllegalRead(address)),
            DATA => {
                let data = try!(memory.ppu_read8(self.address));
                self.increment_address();
                data
            }
            _ => return Err(Error::UnimplementedRead(address)),
        };

        Ok(data)
    }

    pub fn write8<Memory: PpuAddressable>(&mut self, address: Address, data: u8, mut memory: Memory) -> Result<()> {
        self.registers.status |= data & STATUS_LAST_WRITE_MASK;

        match address {
            CONTROLLER => self.registers.controller = data,
            MASK => self.registers.mask = data,
            STATUS => return Err(Error::IllegalWrite(address)),
            OAM_ADDRESS => self.registers.oam_address = data,
            OAM_DATA => {
                self.oam[self.registers.oam_address as usize] = data;
                self.registers.oam_address = self.registers.oam_address.wrapping_add(1);
            }
            SCROLL => {
                match self.scroll_axis {
                    ScrollAxis::X => self.scroll_axis = ScrollAxis::Y,
                    ScrollAxis::Y => {
                        self.scroll_axis = ScrollAxis::X;
                        self.scroll_x = self.registers.scroll;
                        self.scroll_y = data;
                    }
                }
                self.registers.scroll = data;
            }
            ADDRESS => {
                match self.address_phase {
                    AddressPhase::HIGH => self.address_phase = AddressPhase::LOW,
                    AddressPhase::LOW => {
                        self.address_phase = AddressPhase::HIGH;
                        self.address = ((self.registers.address as u16) << 8) | (data as u16);
                    }
                }
                self.registers.address = data;
            }
            DATA => {
                try!(memory.ppu_write8(self.address, data));
                self.increment_address();
            }
            _ => return Err(Error::UnimplementedWrite(address)),
        }
        Ok(())
    }
}
