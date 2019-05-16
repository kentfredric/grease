
In a standard Gentoo repository, a file may exist in `profiles/categories`
which contains a white-list of all known categories shipped in the
repository, in order to distinguish directories containing categories from
directories containing other data, without relying on hard-coded exception
rules in traversal, and without relying on deep directory traversal just
to identify the purpose of a directory.

A [`CategoryFileIterator`] is a low-level tool that provides an
[`Iterator`] over this file, emitting [`Category`] entries for respective
lines, rooted in the passed `root`

# Construction

Creating a [`CategoryFileIterator`] requires two arguments: a `root` to
use as the base for returned [`Category`] objects, and a `file` to read
for category listing.

While there is no inherent requirement that `file` be stored within
`root`, most practical applications will want this to be the case.

Both `root` and `file` may be anything that implements
<code>[Into]\<[PathBuf]\></code>, such as:
* [`String`]
* [`OsString`](std::ffi::OsString)
* [`Path`](std::path::Path)
* <code>[Box]\<[Path](std::path::Path)\></code>
* [`Repository`](crate::repository::Repository)

```no_run
use grease::repository::CategoryFileIterator;
match CategoryFileIterator::for_file(
    "/usr/portage",
    "/usr/portage/profiles/categories",
) {
    Err(e) => panic!(e),
    Ok(iterator) => {
        for category in iterator {
            match category {
                Err(e) => panic!(e),
                Ok(c) => println!("{}", c.name()),
            }
        }
    },
}
```

## Errors

Constructing a [`CategoryFileIterator`] performs various `IO` tasks,
validating and opening the passed `file`, and naturally, this process can
emit errors of its own. ( This is why `for_file` was chosen as a name
instead of `new` )

`for_file` returns a
<code>[Result]\<[CategoryFileIterator],[CategoryFileError]\></code> which
must be handled to see if an error was returned.

* [`CategoryFileError::PathNotFound`]    : The value passed for `file` did
  not exist
* [`CategoryFileError::PathAccessError`] : Some othe kind of IO error
  occurred while trying to
`stat()` or `open()` the passed `file`
* [`CategoryFileError::NotAFile`] : `file` seems to exist alright, but it
  needs to be a file,
not a directory

# Iteration

Most work performed with [`CategoryFileIterator`] will focus around
`.next()`, or any of the methods provided by [`Iterator`] which maps
results from `.next()`

`.next()` Returns an
<code>[`Option`]\<[Result]\<[Category],[CategoryFileError]\>\></code>
where each [`Category`] is built from the provided `root` and the
extracted entry from the `file`, and the aggregate needs to be
destructured before it can be used.

```no_run
use grease::repository::CategoryFileIterator;
let iterator = CategoryFileIterator::for_file(
    "/usr/portage",
    "/usr/portage/profiles/categories",
);
for item_result in iterator.unwrap() {
    println!("{}", item_result.unwrap().name());
}
```
## Errors
The underlying `file` of this iterator is prone to various `OS` emitted
`IO` errors, and when seeing one of these, a [`Result::Err`] is returned,
where `Err` contains a [`CategoryFileError`]:

* [`CategoryFileError::FileDecodeError`] : The underlying layers emitted
  an
[`io::ErrorKind::InvalidData`] which likely indicates malformed UTF8, but
documentation isn't explicitly clear about all possible causes for this.
* [`CategoryFileError::FileReadError`] : Some other kind of `IO` error was
  produced, but we
presently have no better way to identify it other than genericaly, due to
lack of adequate documentation and substantial source indirection
