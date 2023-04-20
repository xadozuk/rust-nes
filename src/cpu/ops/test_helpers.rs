use super::{Op, CpuRegisters, Memory};

pub fn test_op(op: impl Op + 'static) -> (Box<dyn Op>, CpuRegisters, Memory)
{
    (Box::new(op), CpuRegisters::new(), Memory::new())
}