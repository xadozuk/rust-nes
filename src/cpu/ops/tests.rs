pub use super::*;

fn test_op(op: impl Op + 'static) -> (CpuRegisters, Box<dyn Op>)
{
    (CpuRegisters::new(), Box::new(op))
}

mod lda_immediate
{
    use super::*;

    #[test]
    fn test_lda_immediate_simple()
    {
        let (mut r, op) = test_op(LdaImmediate);

        op.call(&mut r, &[0x05]);

        assert_eq!(0x05, *r.a);
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
    }

    #[test]
    fn test_lda_immediate_zero_flag()
    {
        let (mut r, op) = test_op(LdaImmediate);

        op.call(&mut r, &[0x00]);

        assert_eq!(0x00, *r.a);
        assert!(r.p.is_zero());
    }

    #[test]
    fn test_lda_immediate_negative_flag()
    {
        let (mut r, op) = test_op(LdaImmediate);

        op.call(&mut r, &[0b1000_0001]);

        assert_eq!(*r.a, 0b1000_0001);
        assert!(r.p.is_negative());
    }
}

mod tax
{
    use super::*;

    #[test]
    fn test_tax_simple()
    {
        let (mut r, op) = test_op(TransferAToX);

        r.a.set(0x05);

        op.call(&mut r, &[]);

        assert_eq!(*r.x, 0x05);
        assert!(!r.p.is_zero());
        assert!(!r.p.is_negative());
    }

    #[test]
    fn test_tax_zero_flag()
    {
        let (mut r, op) = test_op(TransferAToX);

        r.a.set(0x00);

        op.call(&mut r, &[]);

        assert!(r.p.is_zero());
    }

    #[test]
    fn test_tax_negative_flag()
    {
        let (mut r, op) = test_op(TransferAToX);

        r.a.set(0b1000_0001);

        op.call(&mut r, &[]);

        assert!(r.p.is_negative());
    }
}

mod inx
{
    use super::*;

    #[test]
    fn test_inx_simple()
    {
        let (mut r, op) = test_op(IncrementX);

        r.x.set(0x00);

        op.call(&mut r, &[]);

        assert_eq!(*r.x, 0x01);
        assert!(!r.p.is_negative());
        assert!(!r.p.is_zero());
    }

    #[test]
    fn test_inx_zero_flag()
    {
        let (mut r, op) = test_op(IncrementX);

        r.x.set(0xFF);

        op.call(&mut r, &[]);

        assert_eq!(*r.x, 0x00);
        assert!(r.p.is_zero());
    }

    #[test]
    fn test_inx_negative_flag()
    {
        let (mut r, op) = test_op(IncrementX);

        r.x.set(0b1000_0000);

        op.call(&mut r, &[]);

        assert_eq!(*r.x, 0b1000_0001);
        assert!(r.p.is_negative());
    }
}