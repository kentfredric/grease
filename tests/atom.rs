macro_rules! good_category {
    ($x:expr) => {{
        use grease::atom::Category;
        match $x.parse::<Category>() {
            Ok(cc) => assert_eq!(cc.category(), $x),
            e => panic!("{:?}", e),
        }
    }};
}

macro_rules! bad_category {
    ($x:expr, $y:ident, $z:expr) => {{
        use grease::{atom::Category, err::AtomParseError};
        match $x.parse::<Category>() {
            Err(AtomParseError::$y(c)) => assert_eq!(c, $z),
            e => panic!("{:?}", e),
        }
    }};
}

macro_rules! good_package {
    ($x:expr, $y:expr, $z:expr) => {{
        use grease::atom::Package;
        match $x.parse::<Package>() {
            Ok(cc) => {
                assert_eq!(cc.category(), $y);
                assert_eq!(cc.package(), $z);
            },
            e => panic!("{:?}", e),
        }
    }};
}

macro_rules! bad_package {
    ($x:expr, $y:ident, $z:expr) => {{
        use grease::{atom::Package, err::AtomParseError};
        match $x.parse::<Package>() {
            Err(AtomParseError::$y(c)) => assert_eq!(c, $z),
            e => panic!("{:?}", e),
        }
    }};
}

macro_rules! good_atom {
    ($x:expr, $cat:expr, $pn:expr, $pv:expr, $rv:expr) => {{
        use grease::atom::Atom;
        match $x.parse::<Atom>() {
            Ok(cc) => {
                assert_eq!(cc.category(), $cat);
                assert_eq!(cc.package(), $pn);
                assert_eq!(cc.version(), $pv);
                assert_eq!(cc.revision(), $rv);
            },
            e => panic!("{:?}", e),
        }
    }};
    ($x:expr, $cat:expr, $pn:expr, $pv:expr) => {
        good_atom!($x, $cat, $pn, $pv, None)
    };
    ($x:expr, $cat:expr, $pn:expr, $pv:expr,revision $rv:expr) => {
        good_atom!($x, $cat, $pn, $pv, Some($rv.to_owned()))
    };
}

macro_rules! bad_atom {
    ($x:expr, $y:ident, $z:expr) => {{
        use grease::{atom::Atom, err::AtomParseError};
        match $x.parse::<Atom>() {
            Err(AtomParseError::$y(c)) => assert_eq!(c, $z),
            e => panic!("{:?}", e),
        }
    }};
}

macro_rules! atom_eq {
    ($x:expr, $y:expr) => {{
        use grease::atom::Atom;
        match $x.parse::<Atom>() {
            Ok(left) => match $y.parse::<Atom>() {
                Ok(right) => assert_eq!(left, right),
                e => panic!("{:?}", e),
            },
            e => panic!("{:?}", e),
        }
    }};
}
macro_rules! atom_ne {
    ($x:expr, $y:expr) => {{
        use grease::atom::Atom;
        match $x.parse::<Atom>() {
            Ok(left) => match $y.parse::<Atom>() {
                Ok(right) => assert_ne!(left, right),
                e => panic!("{:?}", e),
            },
            e => panic!("{:?}", e),
        }
    }};
}
macro_rules! atom_cmp {
    ($x:expr, gt $y:expr) => {{
        use grease::atom::Atom;
        use std::cmp::Ordering;
        match $x.parse::<Atom>() {
            Ok(left) => match $y.parse::<Atom>() {
                Ok(right) => match left.partial_cmp(&right) {
                    Some(Ordering::Greater) => assert!(true),
                    e => panic!("{:?} {:?} {:?}" , left, e, right),
                }
                e => panic!("{:?}", e),
            },
            e => panic!("{:?}", e),
        }
    }};
    ($x:expr, lt $y:expr) => {{
        use grease::atom::Atom;
        use std::cmp::Ordering;
        match $x.parse::<Atom>() {
            Ok(left) => match $y.parse::<Atom>() {
                Ok(right) => match left.partial_cmp(&right) {
                    Some(Ordering::Less) => assert!(true),
                    e => panic!("{:?} {:?} {:?}" , left, e, right),
                }
                e => panic!("{:?}", e),
            },
            e => panic!("{:?}", e),
        }
    }};
    ($x:expr, > $y:expr) => {{
        atom_cmp!( $x, gt $y )
    }};
    ($x:expr, < $y:expr) => {{
        atom_cmp!( $x, lt $y )
    }}

}

#[test]
fn parse_category() {
    good_category!("dev-perl");
    good_category!("virtual");
    bad_category!("-invalid", BadCategory, "-invalid");
}

#[test]
fn parse_package() {
    bad_package!("dev-perl", BadAtomPair, "dev-perl");
    bad_package!("virtual", BadAtomPair, "virtual");
    bad_package!("-invalid", BadAtomPair, "-invalid");
    bad_package!("-invalid/name", BadCategory, "-invalid");
    bad_package!("virtual/-invalid", BadPackage, "-invalid");
    good_package!("virtual/valid", "virtual", "valid");

    good_package!("virtual/perl", "virtual", "perl");
    good_package!("virtual/xfwm4", "virtual", "xfwm4");
    good_package!("virtual/xfce4-appfinder", "virtual", "xfce4-appfinder");
    good_package!("virtual/gtk+", "virtual", "gtk+");
    good_package!("virtual/aewm++", "virtual", "aewm++");
    good_package!("virtual/gtk+extra", "virtual", "gtk+extra");
    good_package!("virtual/libstdc++-v3", "virtual", "libstdc++-v3");
    good_package!("virtual/libstdc++-v3-bin", "virtual", "libstdc++-v3-bin");
    good_package!("virtual/rtl_433", "virtual", "rtl_433");
    good_package!("virtual/c++-gtk-utils", "virtual", "c++-gtk-utils");
    good_package!("virtual/mod_perl", "virtual", "mod_perl");
    // Versions in package names are illegal
    bad_package!("virtual/valid-1", BadPackageWithPV, "valid-1");
    bad_package!("virtual/libstdc++-3", BadPackageWithPV, "libstdc++-3");
}

#[test]
fn parse_atom() {
    bad_atom!("dev-perl", BadAtomPair, "dev-perl");
    bad_atom!("virtual", BadAtomPair, "virtual");
    bad_atom!("-invalid", BadAtomPair, "-invalid");
    bad_atom!("-invalid/name", BadCategory, "-invalid");
    bad_atom!("virtual/-invalid", BadPackageVersion, "-invalid");
    good_atom!("virtual/valid-1", "virtual", "valid", "1");
    good_atom!("virtual/valid-10", "virtual", "valid", "10");
    good_atom!("virtual/valid-1.1", "virtual", "valid", "1.1");
    good_atom!("virtual/valid-1.1-r1", "virtual", "valid", "1.1", revision "1");
    good_atom!("virtual/valid-1.1-r0", "virtual", "valid", "1.1", revision "0");
    good_atom!("virtual/valid-1.1_pre", "virtual", "valid", "1.1_pre");
    good_atom!("virtual/valid-1.1_pre2012", "virtual", "valid", "1.1_pre2012");
    good_atom!("virtual/valid-1.1b_pre2012", "virtual", "valid", "1.1b_pre2012");
    good_atom!("virtual/valid-1-r1", "virtual", "valid", "1", revision "1");
    bad_atom!("virtual/valid-1-", BadPackageVersion, "valid-1-");
    bad_atom!("virtual/valid-1.0-", BadPackageVersion, "valid-1.0-");
    bad_atom!("virtual/valid-1.0_r", BadPackageVersion, "valid-1.0_r");
    bad_atom!("virtual/valid-1.0_r1", BadPackageVersion, "valid-1.0_r1");
    bad_atom!("virtual/valid-1.1-r", BadPackageVersion, "valid-1.1-r");
}

#[test]
fn atom_cmp() {
    atom_eq!("dev-lang/perl-5.21.0", "dev-lang/perl-5.21.0");
    atom_ne!("dev-lang/perl-5.21.0", "dev-lang/perl-5.21.1");
    atom_ne!("dev-lang/perl-5.21.0", "virtual/perl-5.21.0");
    atom_ne!("dev-lang/perl-5.21.0", "dev-lang/rust-5.21.0");
    atom_ne!("dev-lang/perl-5.21.0", "dev-lang/perl-5.21.0-r1");
    atom_cmp!("dev-lang/perl-5.21.0", < "dev-lang/perl-5.21.0-r1");
    atom_cmp!("dev-lang/perl-5.21.0", < "dev-lang/perl-5.21.1");
    atom_cmp!("dev-lang/perl-5.21.1", < "dev-lang/perl-5.21.10");
    atom_cmp!("dev-lang/perl-5.21.10", < "dev-lang/perl-5.21.100");
    atom_cmp!("dev-lang/perl-5.21.0", < "dev-lang/perl-5.21.0_pre1");
}
