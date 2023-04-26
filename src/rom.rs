pub const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const PRG_BANK_SIZE: usize = 16384;
pub const CHR_BANK_SIZE: usize = 8192;
pub const PRG_RAM_UNIT:  usize = 8192;

#[derive(Clone, Copy)]
pub enum Mirroring
{
    Horizontal,
    Vertical,
    FourScreen
}

struct RomHeader
{
    prg_banks: usize,
    chr_banks: usize,
    prg_ram_size: usize,
    mapper_number: u8,
    mirroring: Mirroring,
    contains_prg_ram: bool,
    contains_trainer: bool
}

pub struct Rom
{
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring
}

impl Rom
{
    pub fn empty() -> Rom
    {
        Rom
        {
            prg: vec![],
            chr: vec![],
            mapper: 0,
            mirroring: Mirroring::Horizontal
        }
    }

    pub fn from_file(path: &str) -> Result<Rom, String>
    {
        let result = std::fs::read(path);

        match result
        {
            Ok(bytes) => Self::from(bytes),
            Err(err) => Err(format!("Unable to read file '{0}', ({1})", path, err))
        }
    }

    pub fn from(data: Vec<u8>) -> Result<Rom, String>
    {
        let header = Self::parse_header(&data[0..16])?;
        let mapper = header.mapper_number;
        let mirroring = header.mirroring;

        // Skip header and trainer if present
        let prg_rom_start = 16 + if header.contains_trainer { 512 } else { 0 };
        let prg_rom_end = prg_rom_start + header.prg_banks * PRG_BANK_SIZE;

        let chr_rom_start = prg_rom_end;
        let chr_rom_end = chr_rom_start + header.chr_banks * CHR_BANK_SIZE;

        Ok(Rom
        {
            prg: data[prg_rom_start..prg_rom_end].to_vec(),
            chr: data[chr_rom_start..chr_rom_end].to_vec(),
            mapper: mapper,
            mirroring: mirroring
        })
    }

    fn parse_header(header: &[u8]) -> Result<RomHeader, String>
    {
        if header.len() != 16
        {
            return Err(format!("Incorrect header size ({})", header.len()));
        }

        if header[0..4] != NES_TAG
        {
            return Err(String::from("Data header doesn't start with correct NES tag."));
        }

        if header[7] & 0b0000_1111 != 0
        {
            return Err(String::from("NES2.0 format not supported"));
        }

        // Control bits
        let mapper_number = (header[6] & 0b1111_0000) >> 4 | (header[7] & 0b1111) << 4;
        let contains_prg_ram = header[6] & 0b0000_0010 != 0;
        let contains_trainer = header[6] & 0b0000_0100 != 0;

        let mirroring = if header[6] & 0b0000_1000 != 0
        {
            Mirroring::FourScreen
        }
        else
        {
            match header[6] & 0b0000_0001 == 1
            {
                false => Mirroring::Horizontal,
                true  => Mirroring::Vertical
            }
        };

        Ok(RomHeader
        {
            prg_banks: header[4] as usize,
            chr_banks: header[5] as usize,
            prg_ram_size: header[8] as usize,
            mapper_number: mapper_number,
            mirroring: mirroring,
            contains_prg_ram: contains_prg_ram,
            contains_trainer: contains_trainer
        })
    }
}