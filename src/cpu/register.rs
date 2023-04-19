use std::ops::{Deref, AddAssign};

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

pub struct StatusRegister
{
    carry:              bool,
    zero:               bool,
    interrupt_disable:  bool,
    decimal_mode:       bool,
    break_command:      bool,
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
            break_command:      false,
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
        self.break_command =        false;
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

    pub fn set_break(&mut self, flag: bool)
    {
        self.break_command = flag;
    }

    pub fn has_broke(&self) -> bool
    {
        self.break_command
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