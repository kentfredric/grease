#[macro_export]
macro_rules! chain_cmp {
    ($cmp:expr) => {
        $cmp
    };
    ($cmp:expr, $res:expr) => {{
        use std::cmp::Ordering::Equal;
        match $cmp {
            Some(Equal) => $res,
            e => e,
        }
    }};
    ($cmp:expr, $res:expr, $resb:expr) => {
        chain_cmp!($cmp, chain_cmp!($res, $resb))
    };
    ($cmp:expr, $res:expr, $resb:expr, $resc:expr) => {
        chain_cmp!($cmp, $res, chain_cmp!($resb, $resc))
    };
}
