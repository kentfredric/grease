`grease::repository` provides a collection of utilities for iterating and
working with literal filesystem repositories.

```rust
use grease::repository::{Category, Repository};
use std::path::Path;
let r = Repository::new("/usr/portage");
let c = Category::new("/usr/portage", "dev-perl");
assert_eq!(r.path(), Path::new("/usr/portage"));
assert_eq!(c.path(), Path::new("/usr/portage/dev-perl"));
```
