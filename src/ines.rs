use std::{fs, io, result};
use std::io::Read;
use std::ops::Range;

use image::{NesImage, NesHeader, TvSystem, VideoArrangement, PlaychoiceProm};

#[derive(Debug)]
pub enum Error {
    InvalidChecksum,
    InvalidHeader,
    RomTooSmall,
    IoError(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

const HEADER_NUM_BYTES: usize = 16;
const TRAINER_NUM_BYTES: usize = 512;
const PRG_ROM_BLOCK_SIZE: usize = 16384;
const CHR_ROM_BLOCK_SIZE: usize = 8192;
const PLAYCHOICE_NUM_BYTES: usize = 8192;
const PLAYCHOICE_PROM_DATA_NUM_BYTES: usize = 16;
const PLAYCHOICE_PROM_COUNTER_OUT_NUM_BYTES: usize = 16;


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

fn load(header: NesHeader, data: &[u8]) -> NesImage {
    let mut index = 0;

    let trainer = if header.trainer_present {
        let mut v = vec![0; TRAINER_NUM_BYTES];
        v.copy_from_slice(&data[index..(index + TRAINER_NUM_BYTES)]);
        index += TRAINER_NUM_BYTES;
        Some(v)
    } else {
        None
    };

    let prg_rom_num_bytes = header.prg_rom_size as usize *
        PRG_ROM_BLOCK_SIZE;
    let mut prg_rom = vec![0; prg_rom_num_bytes];
    prg_rom.copy_from_slice(&data[index..(index + prg_rom_num_bytes)]);
    index += prg_rom_num_bytes;

    let chr_rom_num_bytes = header.chr_rom_size as usize *
        CHR_ROM_BLOCK_SIZE;
    let mut chr_rom = vec![0; chr_rom_num_bytes];
    chr_rom.copy_from_slice(&data[index..(index + chr_rom_num_bytes)]);
    index += chr_rom_num_bytes;

    let (playchoice_inst_rom,
         playchoice_prom) = if header.playchoice_present {

        let mut inst_rom = vec![0; PLAYCHOICE_NUM_BYTES];
        let mut prom_data = vec![0; PLAYCHOICE_PROM_DATA_NUM_BYTES];
        let mut prom_counter_out = vec![0; PLAYCHOICE_PROM_COUNTER_OUT_NUM_BYTES];

        inst_rom.copy_from_slice(&data[index..(index + PLAYCHOICE_NUM_BYTES)]);
        index += PLAYCHOICE_NUM_BYTES;
        prom_data.copy_from_slice(&data[index..(index + PLAYCHOICE_PROM_DATA_NUM_BYTES)]);
        index += PLAYCHOICE_PROM_DATA_NUM_BYTES;
        prom_counter_out.copy_from_slice(&data[index..(index + PLAYCHOICE_PROM_COUNTER_OUT_NUM_BYTES)]);
        index += PLAYCHOICE_PROM_COUNTER_OUT_NUM_BYTES;

        (Some(inst_rom), Some(PlaychoiceProm {
            data: prom_data,
            counter_out: prom_counter_out,
        }))
    } else {
        (None, None)
    };

    let remaining = data.len() - index;
    let mut extra = vec![0; remaining];
    extra.copy_from_slice(&data[index..]);

    NesImage {
        header: header,
        trainer: trainer,
        prg_rom: prg_rom,
        chr_rom: chr_rom,
        playchoice_inst_rom: playchoice_inst_rom,
        playchoice_prom: playchoice_prom,
        extra: extra,
    }
}

pub fn parse_file(mut file: fs::File) -> Result<NesImage> {
    let mut buffer = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(l) => l,
        Err(e) => return Err(Error::IoError(e)),
    };

    if buffer.len() < HEADER_NUM_BYTES {
        return Err(Error::RomTooSmall);
    }

    let header = try!(parse_header(&buffer[0..HEADER_NUM_BYTES]));

    Ok(load(header, &buffer[HEADER_NUM_BYTES..]))
}

fn parse_header(header: &[u8]) -> Result<NesHeader> {
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

    Ok(NesHeader {
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
