var N = null
 , E = ""
 , T = "t"
 , U = "u"
 , searchIndex = {};

searchIndex["grease"] = {
 "doc": "A Low Level utility kit for performing tasks with Gentoo…"
 , "i": [
  [0, "repository", "grease", "A Representation of a Gentoo repository", null, null]
  , [3, "Repository", "grease::repository", "Represents a gentoo repository", null, null]
  , [4, "RepositoryError", "", "Class of errors producable by Repositories", null, null]
  , [13, "PathNotFound", "", "A specified path did not appear to exist on the filesystem", 0, null]
  , [13, "PathAccessError", "", "A specified path generated IO errors during discovery", 0, null]
  , [13, "NotADir", "", "A specified path was not a directory, but a directory was…", 0, null]
  , [13, "NotAFile", "", "A specified path was not a file, but a file was expected", 0, null]
  , [13, "FileReadError", "", "A specified file had IO errors when reading it", 0, null]
  , [0, "category", "", "Representation of a category in a Gentoo repository", null, null]
  , [3, "Category", "grease::repository::category", "Represents a concrete Gentoo category", null, null]
  , [3, "CategoryFileIterator", "", "Iterate a `categories` file in a portage repository", null, null]
  , [4, "CategoryFileError", "", "Types of errors that can be emitted when creating an…", null, null]
  , [13, "PathNotFound", "", "A specified path did not appear to exist on the filesystem", 1, null]
  , [13, "PathAccessError", "", "A specified path generated IO errors during discovery", 1, null]
  , [13, "NotAFile", "", "A specified path was not a file, but a file was expected", 1, null]
  , [13, "FileDecodeError", "", "An entry in a file encountered decoding errors", 1, null]
  , [13, "FileReadError", "", "An IO error occurred reading a file", 1, null]
  , [11, "new", "", "Construct a new [`Category`] explicitly", 2, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 2, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Return the name of the category", 2, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "for_file", "", "Create a [`CategoryFileIterator`] for a given `file` in a…", 3, [
   [
    ["p"]
   ]
   , [
    ["result", ["categoryfileerror"]]
    , ["categoryfileerror"]
   ]
  ]]
  , [0, "ebuild", "grease::repository", "Representation of an ebuild in a Gentoo repository", null, null]
  , [3, "Ebuild", "grease::repository::ebuild", "Represent a discrete Gentoo ebuild", null, null]
  , [11, "new", "", "Construct a new ebuild explicitly", 4, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns a path to the ebuild file", 4, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [0, "package", "grease::repository", "Representation of a package in a Gentoo repository", null, null]
  , [3, "Package", "grease::repository::package", "Represents a discrete gentoo package", null, null]
  , [11, "new", "", "Construct a new Package Type Object", 5, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Return the path to a gentoo package", 5, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Get the full name of this package", 5, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "new", "grease::repository", "Construct a new Repository trait object", 6, [
   [
    ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 6, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Extract this repositories name from its profiles dir", 6, [
   [
    ["self"]
   ]
   , [
    ["string"]
    , ["result", ["string", "repositoryerror"]]
    , ["repositoryerror"]
   ]
  ]]
  , [11, "to_owned", "", "", 6, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 6, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 6, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 6, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 6, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 6, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 6, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 6, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 6, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "into", "", "", 0, [
   []
   , ["u"]
  ]]
  , [11, "to_string", "", "", 0, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "from", "", "", 0, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 0, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 0, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 0, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 0, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 0, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "as_fail", "", "", 0, [
   [
    ["self"]
   ]
   , ["fail"]
  ]]
  , [11, "to_owned", "grease::repository::category", "", 2, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 2, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 2, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 2, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 2, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 2, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 2, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 2, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 2, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "into", "", "", 3, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 3, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "into_iter", "", "", 3, [
   []
   , ["i"]
  ]]
  , [11, "try_from", "", "", 3, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 3, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 3, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 3, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 3, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "into", "", "", 1, [
   []
   , ["u"]
  ]]
  , [11, "to_string", "", "", 1, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "from", "", "", 1, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 1, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 1, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 1, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 1, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 1, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "as_fail", "", "", 1, [
   [
    ["self"]
   ]
   , ["fail"]
  ]]
  , [11, "to_owned", "grease::repository::ebuild", "", 4, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 4, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 4, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 4, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 4, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 4, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 4, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 4, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 4, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "to_owned", "grease::repository::package", "", 5, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 5, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 5, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 5, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 5, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 5, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 5, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 5, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 5, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "next", "grease::repository::category", "", 3, [
   [
    ["self"]
   ]
   , ["option"]
  ]]
  , [11, "eq", "", "", 2, [
   [
    ["category"]
    , ["self"]
   ]
   , ["bool"]
  ]]
  , [11, "ne", "", "", 2, [
   [
    ["category"]
    , ["self"]
   ]
   , ["bool"]
  ]]
  , [11, "clone", "", "", 2, [
   [
    ["self"]
   ]
   , ["category"]
  ]]
  , [11, "clone", "grease::repository::ebuild", "", 4, [
   [
    ["self"]
   ]
   , ["ebuild"]
  ]]
  , [11, "clone", "grease::repository::package", "", 5, [
   [
    ["self"]
   ]
   , ["package"]
  ]]
  , [11, "clone", "grease::repository", "", 6, [
   [
    ["self"]
   ]
   , ["repository"]
  ]]
  , [11, "as_ref", "grease::repository::category", "", 2, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository::ebuild", "", 4, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository::package", "", 5, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository", "", 6, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "fmt", "grease::repository::category", "", 1, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository", "", 0, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository::category", "", 2, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "", "", 1, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "", "", 3, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository::ebuild", "", 4, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository::package", "", 5, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository", "", 0, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "", "", 6, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "name", "grease::repository::category", "", 1, [
   [
    ["self"]
   ]
   , [
    ["option", ["str"]]
    , ["str"]
   ]
  ]]
  , [11, "cause", "", "", 1, [
   [
    ["self"]
   ]
   , [
    ["option", ["fail"]]
    , ["fail"]
   ]
  ]]
  , [11, "backtrace", "", "", 1, [
   [
    ["self"]
   ]
   , [
    ["backtrace"]
    , ["option", ["backtrace"]]
   ]
  ]]
  , [11, "name", "grease::repository", "", 0, [
   [
    ["self"]
   ]
   , [
    ["option", ["str"]]
    , ["str"]
   ]
  ]]
  , [11, "cause", "", "", 0, [
   [
    ["self"]
   ]
   , [
    ["option", ["fail"]]
    , ["fail"]
   ]
  ]]
  , [11, "backtrace", "", "", 0, [
   [
    ["self"]
   ]
   , [
    ["backtrace"]
    , ["option", ["backtrace"]]
   ]
  ]]
 ]
 , "p": [
  [4, "RepositoryError"]
  , [4, "CategoryFileError"]
  , [3, "Category"]
  , [3, "CategoryFileIterator"]
  , [3, "Ebuild"]
  , [3, "Package"]
  , [3, "Repository"]
 ]
};
initSearch(searchIndex);
addSearchOptions(searchIndex);
