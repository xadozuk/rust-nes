use std::ops::{Deref, AddAssign, SubAssign};

use super::STACK_POINTER_START;

#[derive(Debug)]
pub struct CpuRegisters
{
    pub p:  StatusRegister,
    pub pc: Register<u16>,
    pub sp: Register<u8>,
    pub a:  Register<u8>,
    pub x:  Register<u8>,
    pub y:  Register<u8>,
}

impl CpuRegisters
{
    pub fn new() -> CpuRegisters
    {
        CpuRegisters {
            p: StatusRegister::new(),
            pc: Register::new(),
            sp: StackRegister::from(STACK_POINTER_START),

            a: Register::new(),
            x: Register::new(),
            y: Register::new(),
        }
    }
}


#[derive(Debug)]
pub struct Register<T>
{
    pub value: T,
}

impl<T> Register<T>
    where T: Default + AddAssign<T>
{
    pub fn new() -> Self
    {
        Register { value: T::default() }
    }

    pub fn from(value: T) -> Self
    {
        Register { value: value }
    }

    pub fn set(&mut self, value: T)
    {
        self.value = value;
    }
}

impl<T> AddAssign<T> for Register<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: T)
    {
        self.value += rhs;
    }
}

impl<T> PartialEq<T> for Register<T>
    where T: PartialEq<T>
{
    fn eq(&self, other: &T) -> bool
    {
        self.value == *other
    }
}

impl<T> Deref for Register<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.value
    }
}

type StackRegister = Register<u8>;
impl StackRegister
{
    pub fn increment(&mut self) -> u8
    {
        let value = self.value;
        self.value += 1;

        value
    }

    pub fn decrement(&mut self) -> u8
    {
        let value = self.value;
        self.value -= 1;

        value
    }
}

pub const CARRY_FLAG: u8     = 0b0000_0001;
pub const ZERO_FLAG: u8      = 0b0000_0010;
pub const INTERRUPT_FLAG: u8 = 0b0000_0100;
pub const DECIMAL_FLAG: u8   = 0b0000_1000;
pub const OVERFLOW_FLAG: u8  = 0b0100_0000;
pub const NEGATIVE_FLAG: u8  = 0b1000_0000;

pub const BREAK_FLAG: u8     = 0b0011_0000;
pub const NMI_FLAG: u8       = 0b0010_0000;

#[derive(Debug)]
pub struct StatusRegister
{
    carry:              bool,
    zero:               bool,
    interrupt_disable:  bool,
    decimal_mode:       bool,
    overflow:           bool,
    negative:           bool,
}

impl StatusRegister
{
    pub fn new() -> Self
    {
        StatusRegister {
            carry:              false,
            zero:               false,
            interrupt_disable:  false,
            decimal_mode:       false,
            overflow:           false,
            negative:           false,
        }
    }

    pub fn reset(&mut self)
    {
        self.carry =                false;
        self.zero  =                false;
        self.interrupt_disable =    false;
        self.decimal_mode =         false;
        self.overflow =             false;
        self.negative =             false;
    }

    pub fn update_for_value(&mut self, value : u8)
    {
        self.set_zero(value == 0);
        self.set_negative(value & 0b1000_0000 != 0);
    }

    pub fn set_zero(&mut self, flag: bool)
    {
        self.zero = flag;
    }

    pub fn is_zero(&self) -> bool
    {
        self.zero
    }

    pub fn set_negative(&mut self, flag: bool)
    {
        self.negative = flag;
    }

    pub fn is_negative(&self) -> bool
    {
        self.negative
    }

    pub fn set_carry(&mut self, flag: bool)
    {
        self.carry = flag;
    }

    pub fn has_carry(&self) -> bool
    {
        self.carry
    }

    pub fn set_overflow(&mut self, flag: bool)
    {
        self.overflow = flag;
    }

    pub fn has_overflown(&self) -> bool
    {
        self.overflow
    }

}

impl From<u8> for StatusRegister
{
    fn from(byte: u8) -> Self
    {
        StatusRegister {
            carry:              byte & CARRY_FLAG != 0,
            zero:               byte & ZERO_FLAG != 0,
            interrupt_disable:  byte & INTERRUPT_FLAG != 0,
            decimal_mode:       byte & DECIMAL_FLAG != 0,
            overflow:           byte & OVERFLOW_FLAG != 0,
            negative:           byte & NEGATIVE_FLAG != 0
        }
    }
}

impl From<&StatusRegister> for u8
{
    fn from(reg: &StatusRegister) -> Self
    {
        let mut value = 0;

        if reg.carry                { value |= CARRY_FLAG }
        if reg.zero                 { value |= ZERO_FLAG }
        if reg.interrupt_disable    { value |= INTERRUPT_FLAG }
        if reg.decimal_mode         { value |= DECIMAL_FLAG }
        if reg.overflow             { value |= OVERFLOW_FLAG }
        if reg.negative             { value |= NEGATIVE_FLAG }

        value
    }
}