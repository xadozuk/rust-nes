#[macro_export]
macro_rules! ops
{
    ( $( ($key:expr, $value:expr) ),* ) =>
    {
        {
            let mut map = HashMap::<u8, Box<dyn Op>>::new();
            $(
                map.insert($key, Box::new($value));
            )*
            map
        }
    };
}