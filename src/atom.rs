/*! An atom type for Gentoo packages and parsing tools
!*/

mod regex;
mod rule;

/** A container for aspects of a Portage Atom

A portage [`Atom`] is a unique qualifier that identifies a specfic, unique package, with a specific exact version

It does not support dependency range or equality specifiers
*/

struct Atom {
    category:     String,
    package_name: String,
}

/** A container for aspects of a Portage Package

A portage [`Package`] is a unique qualifier, but without a version, and does not support range or equality specifiers
*/
struct Package {
    category:     String,
    package_name: String,
}

/** A container for aspects of a Portage Category

A portage [`Category`] is a unique qualifier of a *class* of [`Package`]'s,
but without an actual package name or version and does not support range or requality specifiers
*/
struct Category {
    category: String,
}

#[test]
fn valid_category_name_test() {
    assert!(valid_category_name("virtual"));
    assert!(valid_category_name("dev-lang"));
}
#[test]
fn invalid_category_name_test() {
    assert!(!valid_category_name("-illegal"));
}

/** Validate a category name against PMS specification
 **/
pub fn valid_category_name(category_name: &str) -> bool { self::regex::CATEGORY_NAME.is_match(category_name) }

#[test]
fn valid_package_name_test() {
    assert!(valid_package_name("perl"));
    assert!(valid_package_name("xfwm4"));
    assert!(valid_package_name("xfce4-appfinder"));
    assert!(valid_package_name("gtk+"));
    assert!(valid_package_name("aewm++"));
    assert!(valid_package_name("gtk+extra"));
    assert!(valid_package_name("libstdc++-v3"));
    assert!(valid_package_name("libstdc++-v3-bin"));
    assert!(valid_package_name("rtl_433"));
    assert!(valid_package_name("c++-gtk-utils"));
    assert!(valid_package_name("mod_perl"));
}
#[test]
fn invalid_package_name_test() {
    assert!(!valid_package_name("libstdc++-3"));
    assert!(!valid_package_name("-invalid"));
}

/** Validate a package name against PMS specification
 **/
pub fn valid_package_name(package_name: &str) -> bool {
    self::regex::PACKAGE_NAME.is_match(package_name) && !self::regex::VERSION_SUFFIX.is_match(package_name)
}

/** Validate a slot name against PMS specification
 **/
pub fn valid_slot_name(slot_name: &str) -> bool { self::regex::SLOT_NAME.is_match(slot_name) }

/** Validate a USE flag name against PMS specification
 **/
pub fn valid_use_flag_name(use_flag_name: &str) -> bool { self::regex::USE_FLAG_NAME.is_match(use_flag_name) }

#[test]
fn valid_version_test() {
    assert!(valid_version("1"));
    assert!(valid_version("10"));
    assert!(valid_version("1.1"));
    assert!(valid_version("1.1-r1"));
    assert!(valid_version("1.1-r0"));
    assert!(valid_version("1.0_pre"));
    assert!(valid_version("1.0_pre2012"));
    assert!(valid_version("1.0b_pre2012"));
}
#[test]
fn invalid_version_test() {
    assert!(!valid_version("invalid"));
    assert!(!valid_version("1.0-"));
    assert!(!valid_version("1.0_r"));
    assert!(!valid_version("1.0_r1"));
    assert!(!valid_version("1.1-r"));
}

/** Validate a version against PMS specification
 **/
pub fn valid_version(version: &str) -> bool { self::regex::VERSION.is_match(version) }
