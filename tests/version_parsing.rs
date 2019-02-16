extern crate grease;
use grease::version::Version;
use std::ffi::OsString;

#[test]
pub fn basic() -> () {
    Version::new(OsString::from("1234"));
}
