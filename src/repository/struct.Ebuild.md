```rust
# use grease::repository::Ebuild;
# use std::path::Path;
let e = Ebuild::new(
    "/usr/portage",
    "dev-perl",
    "example",
    "example-9999.ebuild",
);
```

# Construction

An [`Ebuild`] is comprised of four aspects: A path to a repository root, a
category name within that root, a package name within that category, and a
file name within that package.

The repository root can be anything that implements
<code>[Into]\<[PathBuf]\></code>, such as:
* [`String`]
* [`OsString`](std::ffi::OsString)
* [`Path`](std::path::Path)
* <code>[Box]\<[Path](std::path::Path)\></code>
* [`Repository`](crate::repository::Repository)

And the category name, package name, and file name can be anything that
implements <code>[Into]\<[Cow]\<'a, [str]\>\></code> such as:

* <code>&[str]</code>
* [`String`]
* <code>&[String]</code>

```rust
# use grease::repository::{Ebuild, Repository};
# use std::path::Path;
Ebuild::new("/usr/portage", "dev-perl", "example", "example-9999.ebuild");
Ebuild::new(
    String::from("/usr/portage"),
    String::from("dev-perl"),
    String::from("example"),
    String::from("example-9999.ebuild"),
);
// Using Repositories implementation of Into<PathBuf>
let r = Repository::new("/usr/portage");
Ebuild::new(&r, "dev-perl", "example", "example-9999.ebuild");
```
