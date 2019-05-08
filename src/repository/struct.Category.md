```rust
# use grease::repository::Category;
# use std::path::Path;
let c = Category::new("/usr/portage", "dev-perl");
assert_eq!(c.path(), Path::new("/usr/portage/dev-perl"));
assert_eq!(c.name(), "dev-perl");
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

# Conversions
A [`Category`] is inherently [`Path`](std::path::Path) based, due to the
implicit conjugation of `root` and `category`. Various convenience
conversions are provided so that you can use a [`Category`] in places you
might want to use a `Path`.

## AsRef\<PathBuf\>
```rust
# use grease::repository::Category;
# use std::path::{Path,PathBuf};
fn demo<P>(path: P) -> ()
where
    P: AsRef<PathBuf>,
{
    assert_eq!(Path::new("/usr/portage/dev-perl"), path.as_ref());
}
demo(Category::new("/usr/portage", "dev-perl"));
```
