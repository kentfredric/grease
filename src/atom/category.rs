/** A container for aspects of a Portage Category

A portage [`Category`] is a unique qualifier of a *class* of [`Package`]'s,
but without an actual package name or version and does not support range or requality specifiers

### Usage
```rust
use grease::atom::Category;

let c: Category = "dev-perl".parse().unwrap();

```
*/

#[derive(Debug, Clone)]
pub struct Category {
    pub(crate) category: String,
}

use crate::{
    atom::{regex, Package},
    err,
};
use std::{cmp::Ordering, str::FromStr};

impl Category {
    /** a getter for this instances category

    */
    pub fn category(&self) -> &str { &self.category }
}

impl FromStr for Category {
    type Err = err::AtomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::err::AtomParseError;

        if regex::CATEGORY_NAME.is_match(s) {
            Ok(Category { category: s.to_owned() })
        } else {
            Err(AtomParseError::BadCategory(s.to_owned()))
        }
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Category) -> bool { self.category == other.category }
}

impl PartialOrd for Category {
    fn partial_cmp(&self, other: &Category) -> Option<Ordering> {
        chain_cmp!(self.category.partial_cmp(&other.category))
    }
}
