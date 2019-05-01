```rust
# use grease::repository::Repository;
# use std::path::Path;
let r = Repository::new("/usr/portage");
assert_eq!(r.path(), Path::new("/usr/portage"));
```

# Construction

The core element of a [`Repository`] is its path. This can either be a
[`PathBuf`] or anything that implements <code>[Into]\<[PathBuf]\></code>,
such as:
* [`String`]
* [`OsString`](std::ffi::OsString)
* [`Path`](std::path::Path)
* <code>[Box]\<[Path](std::path::Path)\></code>

```rust
# use grease::repository::Repository;
# use std::ffi::OsString;
# use std::path::Path;
Repository::new("/usr/portage");
Repository::new(String::from("/usr/portage"));
Repository::new(Path::new("/usr/portage"));
Repository::new(
    Path::new("/usr/portage").to_path_buf().into_boxed_path(),
);
```
