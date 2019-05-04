```rust
# use grease::repository::Category;
let c = Category::new("/usr/portage", "dev-perl");
```

# Construction

A [`Category`] is comprised of two aspects, a path to a repository root,
and the name of the category within that root.

The repository root can be anything that implements
<code>[Into]\<[PathBuf]\></code>, such as:
* [`String`]
* [`OsString`](std::ffi::OsString)
* [`Path`](std::path::Path)
* <code>[Box]\<[Path](std::path::Path)\></code>
* [`Repository`](crate::repository::Repository)

And the category name can be anything that implements
<code>[Into]\<[Cow]\<'a, [str]\>\></code> such as:

* <code>&[str]</code>
* [`String`]
* <code>&[String]</code>

```rust
# use grease::repository::{Category, Repository};
# use std::path::Path;
Category::new("/usr/portage", "dev-perl");
Category::new(String::from("/usr/portage"), String::from("dev-perl"));
// Using Repositories implementation of Into<PathBuf>
let r = Repository::new("/usr/portage");
Category::new(&r, "dev-perl");
```
