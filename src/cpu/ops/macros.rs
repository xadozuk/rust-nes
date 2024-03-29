#[macro_export]
macro_rules! opcodes
{
    ( $( ($opcode:expr, $mode:expr, $value:expr) ),* ) =>
    {
        {
            let mut map = OpcodeMap::new();
            $(
                map.insert($opcode, Opcode
                {
                    opcode: $opcode,
                    mode: $mode,
                    op: Box::new($value)
                });
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! op
{
    ($name:ident) =>
    {
        #[derive(Debug)]
        pub struct $name;
    };
}