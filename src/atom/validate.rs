/*! Utilities for validating various strings against PMS

!*/
use self::super::regex;

#[test]
fn valid_category_name_test() {
    assert!(category_name("virtual"));
    assert!(category_name("dev-lang"));
}
#[test]
fn invalid_category_name_test() {
    assert!(!category_name("-illegal"));
}

/** Validate a category name against PMS specification
 **/
pub fn category_name(category_name: &str) -> bool { regex::CATEGORY_NAME.is_match(category_name) }

#[test]
fn valid_package_name_test() {
    assert!(package_name("perl"));
    assert!(package_name("xfwm4"));
    assert!(package_name("xfce4-appfinder"));
    assert!(package_name("gtk+"));
    assert!(package_name("aewm++"));
    assert!(package_name("gtk+extra"));
    assert!(package_name("libstdc++-v3"));
    assert!(package_name("libstdc++-v3-bin"));
    assert!(package_name("rtl_433"));
    assert!(package_name("c++-gtk-utils"));
    assert!(package_name("mod_perl"));
}
#[test]
fn invalid_package_name_test() {
    assert!(!package_name("libstdc++-3"));
    assert!(!package_name("-invalid"));
}

/** Validate a package name against PMS specification
 **/
pub fn package_name(package_name: &str) -> bool {
    regex::PACKAGE_NAME.is_match(package_name) && !regex::VERSION_SUFFIX.is_match(package_name)
}

/** Validate a slot name against PMS specification
 **/
pub fn slot_name(slot_name: &str) -> bool { regex::SLOT_NAME.is_match(slot_name) }

/** Validate a USE flag name against PMS specification
 **/
pub fn use_flag_name(use_flag_name: &str) -> bool { regex::USE_FLAG_NAME.is_match(use_flag_name) }

#[test]
fn valid_version_test() {
    assert!(version("1"));
    assert!(version("10"));
    assert!(version("1.1"));
    assert!(version("1.1-r1"));
    assert!(version("1.1-r0"));
    assert!(version("1.0_pre"));
    assert!(version("1.0_pre2012"));
    assert!(version("1.0b_pre2012"));
}
#[test]
fn invalid_version_test() {
    assert!(!version("invalid"));
    assert!(!version("1.0-"));
    assert!(!version("1.0_r"));
    assert!(!version("1.0_r1"));
    assert!(!version("1.1-r"));
}

/** Validate a version against PMS specification
 **/
pub fn version(version: &str) -> bool { regex::VERSION.is_match(version) }
