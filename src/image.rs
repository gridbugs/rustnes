#[derive(Debug)]
pub struct NesImage {
    pub header: NesHeader,
    pub trainer: Option<Vec<u8>>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub playchoice_inst_rom: Option<Vec<u8>>,
    pub playchoice_prom: Option<PlaychoiceProm>,
    pub extra: Vec<u8>,
}

#[derive(Debug)]
pub struct NesHeader {
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
    pub prg_ram_size: Option<usize>,
    pub trainer_present: bool,
    pub vs_unisystem_present: bool,
    pub playchoice_present: bool,
    pub nes2_format: bool,
    pub mapper_number: usize,
    pub video_arrangement: VideoArrangement,
    pub tv_system: TvSystem,
}

#[derive(Debug)]
pub struct PlaychoiceProm {
    pub data: Vec<u8>,
    pub counter_out: Vec<u8>,
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
