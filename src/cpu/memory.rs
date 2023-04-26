use crate::rom::Rom;

pub const RAM_START:      u16 = 0x0000;
pub const RAM_END:        u16 = 0xFFFF;
pub const RAM_MIRROR_END: u16 = 0x1FFF;
pub const PPU_START:      u16 = 0x2000;
pub const PPU_MIRROR_END: u16 = 0x3FFF;
pub const PRG_ROM_START:  u16 = 0x8000; // TODO: align with mapper number

pub struct Memory
{
    memory: [u8; 0x10000],
    rom: Rom
}

impl Memory
{
    pub fn new() -> Memory
    {
        Memory {
            memory: [0; 0x10000],
            rom: Rom::empty()
        }
    }

    pub fn load_rom(&mut self, rom: Rom)
    {
        self.rom = rom;
    }

    fn unmirrored_addr(&self, pos: u16) -> usize
    {
        let addr = match pos
        {
            RAM_START..=RAM_MIRROR_END => pos & 0b0111_1111_1111,
            PPU_START..=PPU_MIRROR_END => pos & 0b0010_0000_0000_0111,
            _ => pos
        };

        addr as usize
    }

    pub fn read(&self, pos: u16) -> u8
    {
        match pos
        {
            PRG_ROM_START..=RAM_END => self.rom_read(pos),
            _ => self.memory[self.unmirrored_addr(pos)]
        }
    }

    pub fn write(&mut self, pos: u16, data: u8)
    {
        match pos
        {
            PRG_ROM_START..=RAM_END => panic!("Unable to write into cartridge ROM space"),
            _ => self.memory[self.unmirrored_addr(pos)] = data
        }
    }

    pub fn read_u16(&self, pos: u16) -> u16
    {
        let lsb = self.read(pos);
        let msb = self.read(pos + 1);

        u16::from_le_bytes([lsb, msb])
    }

    pub fn write_u16(&mut self, pos: u16, data: u16)
    {
        let bytes = data.to_le_bytes();

        self.write(pos, bytes[0]);
        self.write(pos + 1, bytes[1]);
    }

    // TODO: handle mapping and prevent writting to rom
    pub fn read_slice(&self, pos: u16, length: usize) -> &[u8]
    {
        &self.memory[(pos as usize)..(pos as usize) + length]
    }

    pub fn write_slice(&mut self, pos: u16, data: &[u8])
    {
        self.memory[(pos as usize)..(pos as usize) + data.len()].copy_from_slice(data);
    }

    fn rom_read(&self, pos: u16) -> u8
    {
        // Reframe only on PRG space
        let mut addr = pos - PRG_ROM_START;

        // Only 1 bank of 16KB PRG so we mirror
        if self.rom.prg.len() == 0x4000 && addr > 0x4000
        {
            addr %= 0x4000;
        }

        self.rom.prg[addr as usize]
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn read()
    {
        let mut m = Memory::new();

        m.memory[0xFF] = 0x01;

        assert_eq!(0x01, m.read(0xFF));
    }

    #[test]
    fn write()
    {
        let mut m = Memory::new();

        m.write(0xFF, 0x01);

        assert_eq!(0x01, m.memory[0xFF]);
    }

    #[test]
    fn read_u16()
    {
        let mut m = Memory::new();

        m.memory[0xFE] = 0x34;
        m.memory[0xFF] = 0x12;

        assert_eq!(0x1234, m.read_u16(0xFE));
    }

    #[test]
    fn write_u16()
    {
        let mut m = Memory::new();

        m.memory[0xFE] = 0x34;
        m.memory[0xFF] = 0x12;

        assert_eq!(0x1234, m.read_u16(0xFE));
    }

    #[test]
    fn read_slice()
    {
        let mut m = Memory::new();

        m.memory[0xA0] = 0x1;
        m.memory[0xA1] = 0x2;
        m.memory[0xA2] = 0x3;

        assert_eq!(&[0x1, 0x2, 0x3], m.read_slice(0xA0, 3));
    }

    #[test]
    fn mirrored_read_ram()
    {
        let mut m = Memory::new();

        m.memory[0x0000] = 0xFF;

        assert_eq!(0xFF, m.read(0x0800));
        assert_eq!(0xFF, m.read(0x1000));
        assert_eq!(0xFF, m.read(0x1800));
    }

    #[test]
    fn mirrored_write_ram()
    {
        let mut m = Memory::new();

        m.write(0x0800, 0xFF);
        assert_eq!(0xFF, m.memory[0x0000]);

        m.write(0x1000, 0xFE);
        assert_eq!(0xFE, m.memory[0x0000]);

        m.write(0x1800, 0xFD);
        assert_eq!(0xFD, m.memory[0x0000]);
    }

    #[test]
    fn mirrored_ppu_read()
    {
        let mut m = Memory::new();

        m.memory[0x2000] = 0xFF;

        assert_eq!(0xFF, m.read(0x2008));
        assert_eq!(0xFF, m.read(0x2010));
    }

    #[test]
    fn mirrored_ppu_write()
    {
        let mut m = Memory::new();

        m.write(0x2008, 0xFF);
        assert_eq!(0xFF, m.memory[0x2000]);

        m.write(0x2010, 0xFE);
        assert_eq!(0xFE, m.memory[0x2000]);

        m.write(0x2018, 0xFD);
        assert_eq!(0xFD, m.memory[0x2000]);
    }

}