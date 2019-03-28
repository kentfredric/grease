use once_cell::{sync::Lazy, sync_lazy};

pub(crate) const CATEGORY_PART: &str = concat!(
    "[A-Za-z0-9_]",     // Leading
    "[A-Za-z0-9+_.-]*", // Rest
);
pub(crate) const PACKAGE_PART: &str = concat!(
    "[A-Za-z0-9_]",     // Leading
    "[A-Za-z0-9+_-]*?"  // Rest
);
pub(crate) const SLOT_PART: &str = concat!(
    "[A-Za-z0-9_]",     // Leading
    "[A-Za-z0-9+_.-]*"  // Rest
);
pub(crate) const USE_FLAG_PART: &str = concat!(
    "[A-Za-z0-9]",      // Leading
    "[A-Za-z0-9+_@-]*"  // Rest
);
pub(crate) const VERSION_PART: &str = concat!(
    "[0-9]+",                              // Leading digit
    "(?:[.][0-9]+)*",                      // dotted digits
    "[a-zA-Z]?",                           // optional tailing letter
    "(?:_(?:alpha|beta|pre|rc|p)[0-9]*)*"  // optional multiple suffixes
);
pub(crate) const VERSION_REVISION_SUFFIX: &str = "(?:-r[0-9]+)";
pub(crate) const DEP_OPERATOR: &str = concat!(
    "=", // Exact match
    "|", ">", // more than
    "|", "<", // less than
    "|", "<=", // lte
    "|", ">=", // gte
    "|", "~", // Any rev
    "|", "!", // Block
    "|", "!!", // Block even temporarily
);
pub(crate) const USE_PREFIX: &str = concat!(
    "-", // Required off
    "|", "!", // Conditional inversion
);
pub(crate) const USE_SUFFIX: &str = concat!(
    r#"\?"#, // Conditional USE-If-Self
    "|", r#"="#, // Conditional MATCH USE
    "|", r#"\(+\)"#, // Default On
    "|", r#"\(-\)"#, // Default Off
);
pub(crate) const SLOT_OPERATOR: &str = concat!(
    r#"\*"#, // Any slot acceptable
    "|", r#"="# // Bind-slot
);

pub(crate) static CATEGORY: Lazy<String> =
    sync_lazy! { format!("^{}$", CATEGORY_PART) };
pub(crate) static PACKAGE: Lazy<String> =
    sync_lazy! {  format!("^{}$", PACKAGE_PART)  };
pub(crate) static SLOT: Lazy<String> =
    sync_lazy! {     format!("^{}$", SLOT_PART)     };
pub(crate) static USE_FLAG: Lazy<String> =
    sync_lazy! { format!("^{}$", USE_FLAG_PART) };
pub(crate) static VERSION: Lazy<String> =
    sync_lazy! {  format!("^{}{}?$", VERSION_PART, VERSION_REVISION_SUFFIX )};
pub(crate) static VERSION_SUFFIX: Lazy<String> = sync_lazy! {
    format!("-{}{}?$", VERSION_PART, VERSION_REVISION_SUFFIX )
};

pub(crate) static USE_FLAG_PARSE: Lazy<String> = sync_lazy! {
    format!(
        "^(?P<prefix>{use_prefix})?(?P<flag>{use_flag})(?P<suffix>{use_suffix})?$",
        use_prefix = &USE_PREFIX,
        use_flag   = &USE_FLAG_PART,
        use_suffix = &USE_SUFFIX,
    )
};

pub(crate) static ATOM_PARSE: Lazy<String> = sync_lazy! {
    format!(
        "^=?(?P<category>{category})/(?P<package>{package})-(?P<version>{version})(?:-r(?P<revision>[0-9]+))?$",
        category = &CATEGORY_PART,
        package  = &PACKAGE_PART,
        version  = &VERSION_PART,
    )
};

pub(crate) static ATOM_SPEC_PARSE_CATEGORY: Lazy<String> = sync_lazy! {
    let op   = format!("(?P<operator>{operator})",
        operator = &DEP_OPERATOR
    );
    let cat =   format!("(?P<category>{category})",
        category = &CATEGORY_PART,
    );
    format!(
        "^{op}?{category}$",
        op = op,
        category = cat,
    )
};

pub(crate) static ATOM_SPEC_PARSE_PNV: Lazy<String> = sync_lazy! {
    let pv = format!(
        "(?:-(?P<version>{version})(?:-r(?P<revision>[0-9]+))?)",
        version = &VERSION_PART,
    );
    let pnv = format!(
        "(?P<package>{package}){version}?",
        package = &PACKAGE_PART,
        version = pv,
    );
    let slot = format!(
        "(?::(?P<slot>{slot})(?P<slot_op>{slot_op})?)",
        slot = &SLOT_PART,
        slot_op = &SLOT_OPERATOR,
    );
    let useflag = format!(
        "({use_prefix})?{use_flag}({use_suffix})?",
        use_prefix = &USE_PREFIX,
        use_flag   = &USE_FLAG_PART,
        use_suffix = &USE_SUFFIX,
    );
    let useflaglist = format!(
        r#"(?:\[(?P<use_flags>{use_flag}(,{use_flag})*)\])"#,
        use_flag = useflag,
    );
    format!(
        "^{pnv}{slot}?{useflags}?$",
        pnv = pnv,
        slot = slot,
        useflags = useflaglist
    )
};

pub(crate) static ATOM_SPEC_PARSE: Lazy<String> = sync_lazy! {
    let op   = format!("(?P<operator>{operator})",
        operator = &DEP_OPERATOR
    );
    let cat  = format!("(?P<category>{category})",
        category = &CATEGORY_PART,
    );
    let pv = format!(
        "(?:-(?P<version>{version})(?:-r(?P<revision>[0-9]+))?)",
        version = &VERSION_PART,
    );
    let pnv = format!(
        "(?P<package>{package}){version}?",
        package = &PACKAGE_PART,
        version = pv,
    );
    let slot = format!(
        "(?::(?P<slot>{slot})(?P<slot_op>{slot_op})?)",
        slot = &SLOT_PART,
        slot_op = &SLOT_OPERATOR,
    );
    let useflag = format!(
        "({use_prefix})?{use_flag}({use_suffix})?",
        use_prefix = &USE_PREFIX,
        use_flag   = &USE_FLAG_PART,
        use_suffix = &USE_SUFFIX,
    );
    let useflaglist = format!(
        r#"\[(?P<use_flags>{use_flag}(,{use_flag})*)\]"#,
        use_flag = useflag,
    );
    format!(
        "^{op}?{category}/{pnv}{slot}?{useflags}?$",
        op = op,
        category = cat,
        pnv = pnv,
        slot = slot,
        useflags = useflaglist
    )
};
