/*! A nom parser for portage atom tokens

!*/

use lazy_static::lazy_static;
use nom::{
    complete, dbg_dmp, do_parse, exact, is_a, many0, many1, many_m_n, named, not, one_of, opt, parse_to, peek,
    re_bytes_find_static, re_find_static, re_match_static, re_matches_static, tuple,
};

named!(
    category_part(&str) -> &str,
    dbg_dmp!(re_find_static!(concat!(
        "^[A-Za-z0-9_]",     // Leading
        "[A-Za-z0-9+_.-]*", // Rest
    )))
);

named!(
    category(&str) -> &str,
    dbg_dmp!(exact!(tuple!(category_part)))
);

named!(
    package_part(&str) -> &str,
    re_find_static!(concat!(
        "[A-Za-z0-9_]",    // Leading
        "[A-Za-z0-9+_-]*"  // Rest
    ))
);

named!(
    slot_part(&str) -> &str,
    re_find_static!(concat!(
        "[A-Za-z0-9_]",     // Leading
        "[A-Za-z0-9+_.-]*"  // Rest
    ))
);

named!(
    use_flag_part(&str) -> &str,
    re_find_static!(concat!(
        "[A-Za-z0-9]",      // Leading
        "[A-Za-z0-9+_@-]*"  // Rest      
    ))
);

named!(
    version_part(&str) -> &str,
    re_find_static!(concat!(
        "[0-9]+",                              // Leading digit
        "(?:[.][0-9]+)*",                      // dotted digits
        "[a-zA-Z]?",                           // optional tailing letter
        "(?:_(?:alpha|beta|pre|rc|p)[0-9]*)*"  // optional multiple suffixes
    ))
);

named!(
    revision_suffix(&str) -> &str,
    re_find_static!(concat!(
         "(?:-r[0-9]+)?"
    ))
);

#[test]
fn test_category() {
    match category("valid") {
        Ok((buf, c)) => {
            assert_eq!(buf, "");
            assert_eq!(c, "valid");
        },
        e => panic!("{:?}", e),
    }

    match category("valid/invalid") {
        Ok(e) => panic!("Should not parse {:?}", e),
        _ => assert!(true),
    }

    match category("-invalid/invalid") {
        Ok((x, y)) => panic!("Shouldn't parse {:?}, {:?}", x, y),
        _ => assert!(true),
    }
}
#[test]
fn test_category_part() {
    match category_part("valid") {
        Ok((x, y)) => {
            assert_eq!(x, "");
            assert_eq!(y, "valid");
        },
        e => panic!("{:?}", e),
    }

    match category_part("valid/invalid") {
        Ok((x, y)) => {
            assert_eq!((x, y), ("/invalid", "valid"));
        },
        e => panic!("Can't extract category prefix {:?}", e),
    }
    match category_part("-invalid/invalid") {
        Ok((x, y)) => panic!("Shouldn't parse {:?}", (x, y)),
        _ => assert!(true),
    }
}
