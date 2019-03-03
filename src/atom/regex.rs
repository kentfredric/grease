use ::once_cell::{sync::Lazy, sync_lazy};
use ::regex::{Regex, RegexBuilder};

/** A regex rule for matching Category Names as [mandated in PMS](https://projects.gentoo.org/pms/6/pms.html#x1-190003.1.1)

> A category name may contain any of the characters `[A-Za-z0-9+_.-]`. It must not begin with a hyphen, a dot or a plus sign.
>
> **Note:** A hyphen is *not* required because of the `virtual` category. Usually, however, category names will contain a hyphen.

*/
pub(crate) static CATEGORY_NAME: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    Regex::new(&fixed_match(
        CATEGORY_FIRST_CHAR.to_owned() + &zero_or_more(CATEGORY_CHARS)
    )).unwrap()
};

/** A regex rule for matching Package Names as [mandated in PMS](https://projects.gentoo.org/pms/6/pms.html#x1-210003.1.2)

> A package name may contain any of the characters `[A-Za-z0-9+_-]`. It must not begin with a hyphen or a plus sign, and must not end in a hyphen followed by anything matching the version syntax described in section 3.2.

> **Note:** A package name does not include the category. The term *qualified package name* is used where a `category/package` pair is meant.

*/

pub(crate) static PACKAGE_NAME: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    Regex::new(&fixed_match(
        PACKAGE_FIRST_CHAR.to_owned() + &zero_or_more(PACKAGE_CHARS)
    )).unwrap()
};

/** A regex rule for matching Slot names as [mandated in PMS](https://projects.gentoo.org/pms/6/pms.html#x1-230003.1.3)

> A slot name may contain any of the characters `[A-Za-z0-9+_.-]`. It must not begin with a hyphen, a dot or a plus sign.
*/

pub(crate) static SLOT_NAME: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    Regex::new(&fixed_match(
            SLOT_FIRST_CHAR.to_owned() + &zero_or_more(SLOT_CHARS)
    )).unwrap()
};

/** A regex rule for matching USE flag names as [mandated in PMS](https://projects.gentoo.org/pms/6/pms.html#x1-240003.1.4)

> A USE flag name may contain any of the characters `[A-Za-z0-9+_@-]`. It must begin with an alphanumeric character. Underscores should be considered reserved for USE_EXPAND, as described in section 11.1.1.
>
> **Note:** The at-sign is required for `LINGUAS`.
*/

pub(crate) static USE_FLAG_NAME: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    Regex::new(&fixed_match(
            USE_FLAG_NAME_FIRST_CHAR.to_owned() + &zero_or_more(USE_FLAG_NAME_CHARS)
    )).unwrap()
};

/** A regex rule for matching against an Atom Version

*/

pub(crate) static VERSION: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    RegexBuilder::new(&fixed_match(
        VERSION_DOTTED_NUM.to_owned()
        + &optional(VERSION_LETTER)
        + &zero_or_more(VERSION_SUFFIX)
        + &optional(VERSION_REVISION_SUFFIX)
    )).ignore_whitespace(true).build().unwrap()
};

pub(crate) static VERSION_SUFFIX: Lazy<Regex> = sync_lazy! {
    use super::rule::*;
    RegexBuilder::new(&(
        "-".to_owned()
        + VERSION_DOTTED_NUM
        + &optional(VERSION_LETTER)
        + &zero_or_more(VERSION_SUFFIX)
        + &optional(VERSION_REVISION_SUFFIX)
        + "$"
    )).ignore_whitespace(true).build().unwrap()
};
