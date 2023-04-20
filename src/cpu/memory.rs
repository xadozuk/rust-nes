pub struct Memory
{
    memory: [u8; 0xFFFF]
}

impl Memory
{
    pub fn new() -> Memory
    {
        Memory {
            memory: [0; 0xFFFF]
        }
    }

    pub fn read(&self, pos: u16) -> u8
    {
        self.memory[pos as usize]
    }

    pub fn write(&mut self, pos: u16, data: u8)
    {
        self.memory[pos as usize] = data;
    }

    pub fn read_u16(&self, pos: u16) -> u16
    {
        let lsb = self.read(pos) as u16;
        let msb = self.read(pos + 1) as u16;

        (msb << 8) + lsb
    }

    pub fn write_u16(&mut self, pos: u16, data: u16)
    {
        let lsb = (data & 0xFF) as u8;
        let msb = (data >> 8) as u8;

        self.write(pos, lsb);
        self.write(pos + 1, msb);
    }

    pub fn read_slice(&self, pos: u16, length: usize) -> &[u8]
    {
        &self.memory[(pos as usize)..(pos as usize) + length]
    }

    pub fn write_slice(&mut self, pos: u16, data: &[u8])
    {
        self.memory[(pos as usize)..(pos as usize) + data.len()].copy_from_slice(data);
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

}