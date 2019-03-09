use once_cell::{sync::Lazy, sync_lazy};

pub(crate) const CATEGORY_PART: &str = concat!(
    "[A-Za-z0-9_]",     // Leading
    "[A-Za-z0-9+_.-]*", // Rest
);
pub(crate) const PACKAGE_PART: &str = concat!(
    "[A-Za-z0-9_]",    // Leading
    "[A-Za-z0-9+_-]*"  // Rest
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

pub(crate) static CATEGORY: Lazy<String> = sync_lazy! { format!("^{}$", CATEGORY_PART) };
pub(crate) static PACKAGE: Lazy<String> = sync_lazy! {  format!("^{}$", PACKAGE_PART)  };
pub(crate) static SLOT: Lazy<String> = sync_lazy! {     format!("^{}$", SLOT_PART)     };
pub(crate) static USE_FLAG: Lazy<String> = sync_lazy! { format!("^{}$", USE_FLAG_PART) };
pub(crate) static VERSION: Lazy<String> = sync_lazy! {  format!("^{}{}?$", VERSION_PART, VERSION_REVISION_SUFFIX )};
pub(crate) static VERSION_SUFFIX: Lazy<String> = sync_lazy! {
    format!("-{}{}?$", VERSION_PART, VERSION_REVISION_SUFFIX )
};

pub(crate) static ATOM_PARSE: Lazy<String> = sync_lazy! {
    format!(
        "^=?(?P<category>{category})/(?P<package>{package})-(?P<version>{version})(?:-r(?P<revision>[0-9]+))?$",
        category = &CATEGORY_PART,
        package  = &PACKAGE_PART,
        version  = &VERSION_PART,
    )
};
