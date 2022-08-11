mod register;
mod ops;

use register::{Register, StatusRegister};
use ops::{Op, Opcodes};

pub struct CpuRegisters
{
    p:  StatusRegister,
    pc: Register<u16>,
    sp: Register<u8>,

    a:  Register<u8>,
    x:  Register<u8>,
    y:  Register<u8>,
}

impl CpuRegisters
{
    pub fn new() -> CpuRegisters
    {
        CpuRegisters {
            p: StatusRegister::new(),
            pc: Register::new(),
            sp: Register::new(),

            a: Register::new(),
            x: Register::new(),
            y: Register::new(),
        }
    }
}

const ROM_START: u16 = 0x8000;

pub struct Cpu
{
    registers: CpuRegisters,
    memory: [u8; 0xFFFF],

    program: Vec<u8>, // TODO: move to RAM,
    opcodes: Opcodes,
}

impl Cpu
{
    pub fn new() -> Cpu
    {
        Cpu {
            registers: CpuRegisters::new(),
            memory: [0; 0xFFFF],

            program: vec![],
            opcodes: ops::ops(),
        }
    }

    pub fn reset(&mut self)
    {
        self.registers.a.set(0);
        self.registers.x.set(0);
        self.registers.y.set(0);
        self.registers.p.reset();

        self.registers.pc.set(
            u16::from_le_bytes([
                self.mem_read(0xFFFC),
                self.mem_read(0xFFFD)
            ])
        );
    }

    pub fn load(&mut self, program: Vec<u8>)
    {
        self.memory[(ROM_START as usize)..(ROM_START as usize + program.len())].copy_from_slice(&program[..]);
        self.memory[0xFFFC..0xFFFD].copy_from_slice(&ROM_START.to_le_bytes());
    }

    pub fn run(&mut self)
    {
        loop
        {
            let opcode = self.program[*self.registers.pc as usize];
            self.registers.pc += 1;

            let opcode = self.opcodes.get(&opcode);

            match opcode
            {
                Some(op) =>
                {
                    let pc = *self.registers.pc as usize;
                    let args = &self.program[pc..(pc+op.args_len())];
                    self.registers.pc += op.args_len() as u16;

                    op.call(&mut self.registers, args);
                },
                None => todo!(),
            }

            if self.registers.p.has_broke() { break }
        }
    }

    fn mem_read(&self, addr: u16) -> u8
    {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8)
    {
        self.memory[addr as usize] = data;
    }
}