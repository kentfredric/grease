`grease::repository` provides a collection of utilities for iterating and
working with literal filesystem repositories.

```rust
use grease::repository::{Category, Ebuild, Package, Repository};
use std::path::Path;
let r = Repository::new("/usr/portage");
let c = Category::new("/usr/portage", "dev-perl");
let p = Package::new("/usr/portage", "dev-perl", "example");
let e = Ebuild::new(
    "/usr/portage",
    "dev-perl",
    "example",
    "example-9999.ebuild",
);
assert_eq!(r.path(), Path::new("/usr/portage"));
assert_eq!(c.path(), Path::new("/usr/portage/dev-perl"));
assert_eq!(c.name(), "dev-perl");
assert_eq!(p.path(), Path::new("/usr/portage/dev-perl/example"));
assert_eq!(p.name(), "dev-perl/example");
assert_eq!(
    e.path(),
    Path::new("/usr/portage/dev-perl/example/example-9999.ebuild")
);
```
