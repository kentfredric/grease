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
// This is legal due to Repository implementing Into<PathBuf>, But you probably just want
// to use Clone/ToOwned
Repository::new(Repository::new("/usr/portage"));
```

# Conversions

Due to [`Repository`]'s being inherently [`Path`](std::path::Path) based,
conversions are provided so you may use them in various
[`Path`](std::path::Path) contexts:

## AsRef\<PathBuf\>
```rust
# use grease::repository::Repository;
# use std::path::{Path,PathBuf};
fn demo<P>(path: P) -> ()
where
  P: AsRef<PathBuf>,
{
  assert_eq!(Path::new("/usr/portage"), path.as_ref());
}
demo(Repository::new("/usr/portage"));
```

## Into\<PathBuf\>
```rust
# use grease::repository::Repository;
# use std::path::{Path, PathBuf};
fn demo<P>(path: P) -> ()
where
  P: Into<PathBuf>,
{
  assert_eq!(Path::new("/usr/portage"), path.into());
}
let r = Repository::new("/usr/portage");
// Using From<&Repository>
demo(&r);
// Using From<Repository>
demo(r);
```
