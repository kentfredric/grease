pub(crate) static CATEGORY_CHARS: &str = "[A-Za-z0-9+_.-]";
pub(crate) static CATEGORY_FIRST_CHAR: &str = "[A-Za-z0-9_]";
pub(crate) static PACKAGE_CHARS: &str = "[A-Za-z0-9+_-]";
pub(crate) static PACKAGE_FIRST_CHAR: &str = "[A-Za-z0-9_]";
pub(crate) static SLOT_CHARS: &str = "[A-Za-z0-9+_.-]";
pub(crate) static SLOT_FIRST_CHAR: &str = "[A-Za-z0-9_]";
pub(crate) static USE_FLAG_NAME_CHARS: &str = "[A-Za-z0-9+_@-]";
pub(crate) static USE_FLAG_NAME_FIRST_CHAR: &str = "[A-Za-z0-9]";
pub(crate) static VERSION_DOTTED_NUM: &str = r##"[0-9]+(?:\.[0-9]+)*"##;
pub(crate) static VERSION_LETTER: &str = "[a-zA-Z]";
pub(crate) static VERSION_SUFFIX: &str = "(?:_(?:alpha|beta|pre|rc|p)[0-9]*)";
pub(crate) static VERSION_REVISION_SUFFIX: &str = "(?:-r[0-9]+)";

pub(crate) fn zero_or_more<S: AsRef<str>>(subject: S) -> String { String::new() + subject.as_ref() + "*" }
pub(crate) fn fixed_match<S: AsRef<str>>(subject: S) -> String { String::new() + "^" + subject.as_ref() + "$" }
pub(crate) fn optional<S: AsRef<str>>(subject: S) -> String { String::new() + subject.as_ref() + "?" }
