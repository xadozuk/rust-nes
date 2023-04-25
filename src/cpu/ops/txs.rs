use super::{Op, AddressingMode, CpuRegisters, Memory};

op!(Txs);
impl Op for Txs
{
    fn call(&self, _: AddressingMode, registers: &mut CpuRegisters, _: &mut Memory)
    {
        let value = *registers.x;

        registers.sp.set(value);
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
        let (op, mut r, mut m) = test_op(Txs);

        r.x.set(0x10);

        op.call(AddressingMode::Implicit, &mut r, &mut m);

        assert_eq!(0x10, *r.sp);
    }
}