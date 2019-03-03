use grease::version;

#[test]
pub fn simple_pv_without_r() -> () {
    let v = version::parse("1234");
    assert_eq!(v.pv(), "1234");
    assert_eq!(v.pr(), "r0");
}

#[test]
pub fn simple_pv_with_r() -> () {
    let v = version::parse("1234-r1");
    assert_eq!(v.pv(), "1234");
    assert_eq!(v.pr(), "r1");
}
