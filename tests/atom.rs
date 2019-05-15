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

macro_rules! good_atom_spec {
    ($x:expr) => {{
        use failure::Error;
        use grease::atom::AtomSpec;
        match $x.parse::<AtomSpec>() {
            Ok(cc) => {
                println!(":) {:?}", cc);
            },
            Err(f) => {
                let e: Error = f.into();
                panic!("{:?}", e)
            },
        }
    }};
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

macro_rules! bad_atom_spec {
    ($x:expr, $y:ident, $aa:expr) => {{
        use grease::{atom::AtomSpec, err::AtomParseError};
        match $x.parse::<AtomSpec>() {
            Err(AtomParseError::$y(a)) => {
                assert_eq!(a, $aa);
            },
            e => panic!("{:?}", e),
        }
    }};

    ($x:expr, $y:ident, $aa:expr, $bb:expr) => {{
        use grease::{atom::AtomSpec, err::AtomParseError};
        match $x.parse::<AtomSpec>() {
            Err(AtomParseError::$y(a, b)) => {
                assert_eq!(a, $aa);
                assert_eq!(b, $bb);
            },
            e => panic!("{:?}", e),
        }
    }};

    ($x:expr, $y:ident, $aa:expr, $bb:expr, $cc:expr) => {{
        use grease::{atom::AtomSpec, err::AtomParseError};
        match $x.parse::<AtomSpec>() {
            Err(AtomParseError::$y(a, b, c)) => {
                assert_eq!(a, $aa);
                assert_eq!(b, $bb);
                assert_eq!(c, $cc);
            },
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
                    Some(Ordering::Greater) => (),
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
                    Some(Ordering::Less) => (),
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

macro_rules! assert_cmp {
    (== $x:expr, $op:tt $y:expr) => {{
        if $x $op $y {
            ()
        } else {
            panic!("{:?} !{} {:?}", $x, stringify!($op), $y)
        }
    }};
    (literal $x:expr, $op:tt literal $y:expr) => {{
        if $x $op $y {
            ()
        } else {
            panic!("{:?} !{} {:?}", $x, stringify!($op), $y)
        }
    }};
    (+ $x:expr, $op:tt $y:expr) => {{
        assert_cmp!(== $x, $op $y)
    }};
    (+ $x:expr, $op:tt $y:expr => $ytype:ty) => {{
        assert_cmp!(== $x, $op format!("{}", $y.parse::<$ytype>().unwrap()))
    }};
    (literal $x:expr, $op:tt $y:expr) => {{
        assert_cmp!(== $x, $op $y.unwrap())
    }};
    ($x:expr, $op:tt literal $y:expr) => {{
        assert_cmp!(== $x.unwrap(), $op $y)
    }};
    ($x:expr, $op:tt $y:expr) => {{
        assert_cmp!(== $x.unwrap(), $op $y.unwrap())
    }};
    ($x:expr => $xtype:ty, $op:tt $yexpr:expr => $ytype:ty) => {{
        assert_cmp!(== $x.parse::<$xtype>().unwrap(), $op $y.parse::<$ytype>().unwrap())
    }};
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
fn parse_atom_spec() {
    good_atom_spec!("dev-lang/perl");
    good_atom_spec!("=dev-lang/perl-5.21.0");
    good_atom_spec!("=dev-lang/perl-5.21.0:0");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=[ithreads]");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=[!ithreads]");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=[!ithreads?]");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=[-ithreads?]");
    good_atom_spec!("=dev-lang/perl-5.21.0:0=[-ithreads?,debug]");
    bad_atom_spec!("=dev-lang/perl", IllegalOperator, "=", "=dev-lang", "=dev-lang/perl");
    bad_atom_spec!("dev-lang/perl-5.21.0", IllegalVersion, "5.21.0", "perl-5.21.0", "dev-lang/perl-5.21.0");
    bad_atom_spec!("dev-perl/Moose-5-r0", IllegalVersion, "5", "Moose-5-r0", "dev-perl/Moose-5-r0");
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

    use grease::atom::{Atom, AtomSpec, Category, Package};

    assert_cmp!("dev-lang/perl-5.20".parse::<Atom>(), < "=dev-lang/perl-5.21".parse::<AtomSpec>());
    assert_cmp!("dev-lang/perl-5.20".parse::<Atom>(), < "=dev-lang/perl-5.20".parse::<AtomSpec>());
    assert_cmp!("dev-lang/perl-5.20-r1".parse::<Atom>(), < "=dev-lang/perl-5.20-r1".parse::<AtomSpec>());
    assert_cmp!("dev-lang/perl-5.20-r2".parse::<Atom>(), > "=dev-lang/perl-5.20-r1".parse::<AtomSpec>());
    assert_cmp!("dev-lang/perl-5.20".parse::<Atom>(), < "=dev-lang/perl-5.20-r1".parse::<AtomSpec>());

    assert_cmp!("dev-lang/perl-5.20".parse::<Atom>(), > "dev-lang/perl".parse::<AtomSpec>());

    assert_cmp!("dev-lang/perl-5.20".parse::<Atom>(), < "dev-lang/perl-5.21".parse::<Atom>());
    assert_cmp!("dev-lang/perl".parse::<Package>(), < "dev-lang/perl-5.21".parse::<Atom>());
    assert_cmp!("dev-lang".parse::<Category>(), < "dev-lang/perl-5.21".parse::<Atom>());
}

#[test]
fn atom_display() {
    use grease::atom::{Atom, AtomSpec, Category, Package};
    assert_cmp!(+ "dev-perl", != "dev-lang" => Category);
    assert_cmp!(+ "dev-perl", == "dev-perl" => Category);

    assert_cmp!(+ "dev-perl/perl",  != "dev-lang/Moose" => Package);
    assert_cmp!(+ "dev-perl/Moose", == "dev-perl/Moose" => Package);

    assert_cmp!(+ "dev-perl/perl-5",  != "dev-lang/Moose-5" => Atom);
    assert_cmp!(+ "dev-perl/Moose-5", == "dev-perl/Moose-5" => Atom);
    // Leading = is supported for Atoms but stripped in output
    assert_cmp!(+ "dev-perl/Moose-5", == "=dev-perl/Moose-5" => Atom);
    // -r0 should not == no -r
    assert_cmp!(+ "dev-perl/Moose-5",    != "dev-perl/Moose-5-r0" => Atom);
    assert_cmp!(+ "dev-perl/Moose-5-r0", != "dev-perl/Moose-5-r1" => Atom);
    assert_cmp!(+ "dev-perl/Moose-5-r0", == "dev-perl/Moose-5-r0" => Atom);

    assert_cmp!(+ "dev-perl/Moose-5",    != "=dev-perl/Moose-5-r0" => AtomSpec);
    assert_cmp!(+ "dev-perl/Moose-5-r0", != "=dev-perl/Moose-5-r1" => AtomSpec);
    assert_cmp!(+ "=dev-perl/Moose-5-r0", == "=dev-perl/Moose-5-r0" => AtomSpec);

    assert_cmp!(+ "=dev-perl/Moose-5-r0:0",  == "=dev-perl/Moose-5-r0:0"  => AtomSpec);
    assert_cmp!(+ "=dev-perl/Moose-5-r0:0*", == "=dev-perl/Moose-5-r0:0*" => AtomSpec);
    assert_cmp!(+ "dev-perl/Moose:0*",       == "dev-perl/Moose:0*"       => AtomSpec);
    assert_cmp!(+ "dev-perl/Moose:0*[foo]",  == "dev-perl/Moose:0*[foo]"  => AtomSpec);

    assert_cmp!(+ "dev-perl/Moose:0*[foo,-bar]",
                == "dev-perl/Moose:0*[foo,-bar]"                => AtomSpec);

    assert_cmp!(+ "dev-perl/Moose:0*[foo,-bar,!baz?]",
                == "dev-perl/Moose:0*[foo,-bar,!baz?]"          => AtomSpec);

    assert_cmp!(+ "dev-perl/Moose:0*[foo,-bar,!baz?,quux?]",
                == "dev-perl/Moose:0*[foo,-bar,!baz?,quux?]"    => AtomSpec);
}
