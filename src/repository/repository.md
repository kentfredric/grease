`grease::repository` provides a collection of utilities for iterating and
working with literal filesystem repositories.

```rust
use grease::repository::Repository;
use std::path::Path;
let r = Repository::new("/usr/portage");
assert_eq!(r.path(), Path::new("/usr/portage"));
```
