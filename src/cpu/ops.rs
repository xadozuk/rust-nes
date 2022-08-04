#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use super::CpuRegisters;

pub type Opcodes = HashMap<u8, Box<dyn Op>>;

pub trait Op
{
    fn call(&self, registers: &mut CpuRegisters, args: &[u8]);
    fn len(&self) -> usize;

    fn args_len(&self) -> usize
    {
        self.len() - 1
    }
}


pub fn ops() -> Opcodes
{
    ops!(
        (0x00, BreakOp),
        (0xA9, LdaImmediate)
    )
}

struct BreakOp;
impl Op for BreakOp
{
    fn call(&self, registers: &mut CpuRegisters, _: &[u8])
    {
        registers.p.set_break(true);
    }

    fn len(&self) -> usize
    {
        1
    }
}

struct LdaImmediate;
impl Op for LdaImmediate
{
    fn call(&self, registers: &mut CpuRegisters, args: &[u8])
    {
        registers.a.set(args[0]);
        registers.p.update_for_value(args[0]);
    }

    fn len(&self) -> usize
    {
        2
    }
}

struct TransferAToX;
impl Op for TransferAToX
{
    fn call(&self, registers: &mut CpuRegisters, _: &[u8])
    {
        registers.x.set(*registers.a);
        registers.p.update_for_value(*registers.a);
    }

    fn len(&self) -> usize
    {
        1
    }
}

struct IncrementX;
impl Op for IncrementX
{
    fn call(&self, registers: &mut CpuRegisters, _: &[u8])
    {
        registers.x.set(registers.x.wrapping_add(1));
        registers.p.update_for_value(*registers.x);
    }

    fn len(&self) -> usize
    {
        1
    }
}