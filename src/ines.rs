use std::{fs, io, result};
use std::io::Read;
use std::ops::Range;

#[derive(Debug)]
pub enum Error {
    InvalidChecksum,
    InvalidHeader,
    RomTooSmall,
    IoError(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

pub struct Ines {
    pub header: InesHeader,
}

#[derive(Debug)]
pub struct InesHeader {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub prg_ram_size: Option<u8>,
    pub trainer_present: bool,
    pub vs_unisystem_present: bool,
    pub playchoice_present: bool,
    pub nes2_format: bool,
    pub mapper_number: u8,
    pub video_arrangement: VideoArrangement,
    pub tv_system: TvSystem,
}

#[derive(Debug)]
pub enum VideoArrangement {
    HorizontalMirroring,
    VerticalMirroring,
    FourScreenVram,
}

#[derive(Debug)]
pub enum TvSystem {
    Ntsc,
    Pal,
}

const HEADER_NUM_BYTES: usize = 16;

// Header fields
const HEADER_CHECKSUM: Range<usize> = Range { start: 0, end: 4 };
const HEADER_PRG_ROM_SIZE: usize = 4;
const HEADER_CHR_ROM_SIZE: usize = 5;
const HEADER_FLAGS_6: usize = 6;
const HEADER_FLAGS_7: usize = 7;
const HEADER_PRG_RAM_SIZE: usize = 8;
const HEADER_FLAGS_9: usize = 9;
const HEADER_FLAGS_10: usize = 10;
const HEADER_ZERO_FILLED: Range<usize> = Range { start: 11, end: 16 };

// header bit flags

const FLAGS_6_SCREEN_LOW_BIT: u8 = 0;
const FLAGS_6_PRG_RAM_BIT: u8 = 1;
const FLAGS_6_TRAINER_BIT: u8 = 2;
const FLAGS_6_SCREEN_HIGH_BIT: u8 = 3;
const FLAGS_6_MAPPER_NUMBER_LOW_OFFSET: u8 = 4;

const FLAGS_7_VS_UNISYSTEM_BIT: u8 = 0;
const FLAGS_7_PLAYCHOICE_BIT: u8 = 1;
const FLAGS_7_NES2_LOW_BIT: u8 = 2;
const FLAGS_7_NES2_HIGH_BIT: u8 = 3;
const FLAGS_7_MAPPER_NUMBER_HIGH_OFFSET: u8 = 4;

const FLAGS_9_TV_SYSTEM_PAL_BIT: u8 = 0;

impl Ines {
    pub fn parse_file(mut file: fs::File) -> Result<Self> {
        let mut buffer = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(l) => l,
            Err(e) => return Err(Error::IoError(e)),
        };

        if buffer.len() < HEADER_NUM_BYTES {
            return Err(Error::RomTooSmall);
        }

        let header = try!(InesHeader::parse(&buffer[0..HEADER_NUM_BYTES]));

        Ok(Ines {
            header: header,
        })
    }
}

impl InesHeader {
    fn parse(header: &[u8]) -> Result<Self> {
        // validate checksum
        if &header[HEADER_CHECKSUM] != [0x4e, 0x45, 0x53, 0x1a] {
            return Err(Error::InvalidChecksum);
        }

        // check zero-filled end of header
        if &header[HEADER_ZERO_FILLED] != [0, 0, 0, 0, 0] {
            return Err(Error::InvalidHeader);
        }

        let flags6 = header[HEADER_FLAGS_6];
        let flags7 = header[HEADER_FLAGS_7];
        let flags9 = header[HEADER_FLAGS_9];

        let video_arrangement = if flags6 & (1 << FLAGS_6_SCREEN_HIGH_BIT) != 0 {
            VideoArrangement::FourScreenVram
        } else if flags6 & (1 << FLAGS_6_SCREEN_LOW_BIT) != 0 {
            VideoArrangement::VerticalMirroring
        } else {
            VideoArrangement::HorizontalMirroring
        };

        let prg_ram_size = if flags6 & (1 << FLAGS_6_PRG_RAM_BIT) != 0 {
            Some(header[HEADER_PRG_RAM_SIZE])
        } else {
            None
        };

        let trainer_present = flags6 & (1 << FLAGS_6_TRAINER_BIT) != 0;
        let playchoice_present = flags7 & (1 << FLAGS_7_PLAYCHOICE_BIT) != 0;
        let vs_unisystem_present = flags7 & (1 << FLAGS_7_VS_UNISYSTEM_BIT) != 0;

        let mapper_number_low = flags6 >> FLAGS_6_MAPPER_NUMBER_LOW_OFFSET;
        let mapper_number_high = flags7 >> FLAGS_7_MAPPER_NUMBER_HIGH_OFFSET;
        let mapper_number = mapper_number_low | (mapper_number_high << 4);

        let nes2_format = ((flags7 & ((1 << FLAGS_7_NES2_LOW_BIT) |
                                      (1 << FLAGS_7_NES2_HIGH_BIT))) >>
            FLAGS_7_NES2_LOW_BIT) == 2;

        let tv_system = if flags9 & (1 << FLAGS_9_TV_SYSTEM_PAL_BIT) != 0 {
            TvSystem::Pal
        } else {
            TvSystem::Ntsc
        };

        Ok(InesHeader {
            prg_rom_size: header[HEADER_PRG_ROM_SIZE],
            chr_rom_size: header[HEADER_CHR_ROM_SIZE],
            prg_ram_size: prg_ram_size,
            trainer_present: trainer_present,
            mapper_number: mapper_number,
            playchoice_present: playchoice_present,
            vs_unisystem_present: vs_unisystem_present,
            nes2_format: nes2_format,
            video_arrangement: video_arrangement,
            tv_system: tv_system,
        })
    }
}
