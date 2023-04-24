use super::{Op, AddressingMode, CpuRegisters, Memory};

pub struct Pha;
impl Op for Pha
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, memory: &mut Memory)
    {
        self.stack_push(registers, memory, *registers.a);
    }
}

#[cfg(test)]
mod tests
{
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn simple()
    {
        let (op, mut r, mut m) = test_op(Pha);

        r.a.set(0x50);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x50, op.stack_peek(&r, &m));
        assert_eq!(0xFE, *r.sp);
    }
}