```rust
# use grease::repository::Package;
# use std::path::Path;
let p = Package::new("/usr/portage", "dev-perl", "example");
assert_eq!(p.path(), Path::new("/usr/portage/dev-perl/example"));
assert_eq!(p.name(), "dev-perl/example");
```

# Construction

A [`Package`] is comprised of three aspects: a path to a repository root,
the name of its category within that root, and the name of its package
within that category.

A repository root can be anything that implements
<code>[Into]\<[PathBuf]\></code>, such as:
* [`String`]
* [`OsString`](std::ffi::OsString)
* [`Path`](std::path::Path)
* <code>[Box]\<[Path](std::path::Path)\></code>
* [`Repository`](crate::repository::Repository)

And the category and package names can be anything that implements
<code>[Into]\<[Cow]\<'a, [str]\>\></code> such as:

* <code>&[str]</code>
* [`String`]
* <code>&[String]</code>

```rust
# use grease::repository::{Package, Repository};
# use std::path::Path;
Package::new("/usr/portage", "dev-perl", "example");
Package::new(
    String::from("/usr/portage"),
    String::from("dev-perl"),
    String::from("example"),
);
// Using Repositories implementation of Into<PathBuf>
let r = Repository::new("/usr/portage");
Package::new(&r, "dev-perl", "example");
```

# Conversions

Due to [`Package`] being inherently [`Path`](std::path::Path) based,
conversions are provided so you may use them in various
[`Path`](std::path::Path) contexts:

## AsRef\<PathBuf\>
```rust
# use grease::repository::Package;
# use std::path::{Path,PathBuf};
fn demo<P>(path: P) -> ()
where
    P: AsRef<PathBuf>,
{
    assert_eq!(Path::new("/usr/portage/dev-perl/example"), path.as_ref());
}
demo(Package::new("/usr/portage", "dev-perl", "example"));
```

## Into\<PathBuf\>
```rust
# use grease::repository::Package;
# use std::path::{Path,PathBuf};
fn demo<P>(path: P) -> ()
where
    P: Into<PathBuf>,
{
    assert_eq!(Path::new("/usr/portage/dev-perl/example"), path.into());
}
let p = Package::new("/usr/portage", "dev-perl", "example");
// Using From<&Package>
demo(&p);
// Using From<Package>
demo(p);
```
