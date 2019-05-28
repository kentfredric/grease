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
  , [3, "CategoryDirsIterator", "", "Iterate categories in a portage repository via discovery", null, null]
  , [4, "CategoryFileError", "", "Types of errors that can be emitted when creating an…", null, null]
  , [13, "PathNotFound", "", "A specified path did not appear to exist on the filesystem", 1, null]
  , [13, "PathAccessError", "", "A specified path generated IO errors during discovery", 1, null]
  , [13, "NotAFile", "", "A specified path was not a file, but a file was expected", 1, null]
  , [13, "FileDecodeError", "", "An entry in a file encountered decoding errors", 1, null]
  , [13, "FileReadError", "", "An IO error occurred reading a file", 1, null]
  , [4, "CategoryDirsError", "", "Types of errors that can be emitted when creating an…", null, null]
  , [13, "RepoNotFound", "", "A specified repository root was not found", 2, null]
  , [13, "RepoAccessError", "", "An IO error occurred accessing the repository root", 2, null]
  , [13, "RepoNotADir", "", "A specified repository root was not a dir, but a dir was…", 2, null]
  , [13, "RepoReadDirError", "", "An error occurred in invoking readdir()", 2, null]
  , [13, "CategoryNameDecodeError", "", "An error occurred decoding a filename to Unicode", 2, null]
  , [13, "CategoryNotFound", "", "A discovered category in root was not found", 2, null]
  , [13, "CategoryAccessError", "", "A discovered category in root had IO Access Errors", 2, null]
  , [13, "CategoryPathLacksFileName", "", "Weirdly, the category iterator passed a PathBuf for…", 2, null]
  , [13, "PackageAccessError", "", "A discovered cat/pn file had IO Access Errors", 2, null]
  , [13, "EbuildAccessError", "", "A discovered cat/pn file had IO Access Errors", 2, null]
  , [13, "EbuildNameDecodeError", "", "An error with decoding a file within a cat/pn dir", 2, null]
  , [13, "CategoryReadDirError", "", "An error occurred in invoking readdir() on a category", 2, null]
  , [4, "CategoryMatcher", "", "Kinds of match rules for category validation", null, null]
  , [13, "ExcludeBlackListed", "", "A matcher rule for categories that excludes members from a…", 3, null]
  , [13, "RequireEbuild", "", "A matcher rule for categories that excludes members if the…", 3, null]
  , [11, "new", "", "Construct a new [`Category`] explicitly", 4, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 4, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Return the name of the category", 4, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "for_file", "", "Create a [`CategoryFileIterator`] for a given `file` in a…", 5, [
   [
    ["p"]
   ]
   , [
    ["result", ["categoryfileerror"]]
    , ["categoryfileerror"]
   ]
  ]]
  , [11, "select", "", "Apply the given matcher against a category at path…", 3, [
   [
    ["p"]
   ]
   , [
    ["result", ["bool", "categorydirserror"]]
    , ["categorydirserror"]
    , ["bool"]
   ]
  ]]
  , [11, "for_repo", "", "Construct a discovery based category iterator for a…", 6, [
   [
    ["p"]
   ]
   , [
    ["result", ["categorydirserror"]]
    , ["categorydirserror"]
   ]
  ]]
  , [0, "ebuild", "grease::repository", "Representation of an ebuild in a Gentoo repository", null, null]
  , [3, "Ebuild", "grease::repository::ebuild", "Represent a discrete Gentoo ebuild", null, null]
  , [11, "new", "", "Construct a new ebuild explicitly", 7, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns a path to the ebuild file", 7, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [0, "package", "grease::repository", "Representation of a package in a Gentoo repository", null, null]
  , [3, "Package", "grease::repository::package", "Represents a discrete gentoo package", null, null]
  , [11, "new", "", "Construct a new Package Type Object", 8, [
   [
    ["s"]
    , ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Return the path to a gentoo package", 8, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Get the full name of this package", 8, [
   [
    ["self"]
   ]
   , ["string"]
  ]]
  , [11, "new", "grease::repository", "Construct a new Repository trait object", 9, [
   [
    ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 9, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Extract this repositories name from its profiles dir", 9, [
   [
    ["self"]
   ]
   , [
    ["result", ["string", "repositoryerror"]]
    , ["string"]
    , ["repositoryerror"]
   ]
  ]]
  , [11, "to_owned", "", "", 9, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 9, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 9, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 9, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 9, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 9, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 9, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 9, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 9, [
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
  , [11, "to_owned", "grease::repository::category", "", 4, [
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
  , [11, "into_iter", "", "", 5, [
   []
   , ["i"]
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
  , [11, "into_iter", "", "", 6, [
   []
   , ["i"]
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
  , [11, "into", "", "", 2, [
   []
   , ["u"]
  ]]
  , [11, "to_string", "", "", 2, [
   [
    ["self"]
   ]
   , ["string"]
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
  , [11, "as_fail", "", "", 2, [
   [
    ["self"]
   ]
   , ["fail"]
  ]]
  , [11, "to_owned", "", "", 3, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 3, [
   [
    ["t"]
    , ["self"]
   ]
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
  , [11, "to_owned", "grease::repository::ebuild", "", 7, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 7, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 7, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 7, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 7, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 7, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 7, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 7, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 7, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "to_owned", "grease::repository::package", "", 8, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 8, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 8, [
   []
   , ["u"]
  ]]
  , [11, "from", "", "", 8, [
   [
    ["t"]
   ]
   , ["t"]
  ]]
  , [11, "try_from", "", "", 8, [
   [
    ["u"]
   ]
   , ["result"]
  ]]
  , [11, "try_into", "", "", 8, [
   []
   , ["result"]
  ]]
  , [11, "borrow", "", "", 8, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "borrow_mut", "", "", 8, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "type_id", "", "", 8, [
   [
    ["self"]
   ]
   , ["typeid"]
  ]]
  , [11, "next", "grease::repository::category", "", 5, [
   [
    ["self"]
   ]
   , ["option"]
  ]]
  , [11, "next", "", "", 6, [
   [
    ["self"]
   ]
   , ["option"]
  ]]
  , [11, "eq", "", "", 4, [
   [
    ["self"]
    , ["category"]
   ]
   , ["bool"]
  ]]
  , [11, "ne", "", "", 4, [
   [
    ["self"]
    , ["category"]
   ]
   , ["bool"]
  ]]
  , [11, "clone", "", "", 4, [
   [
    ["self"]
   ]
   , ["category"]
  ]]
  , [11, "clone", "", "", 3, [
   [
    ["self"]
   ]
   , ["categorymatcher"]
  ]]
  , [11, "clone", "grease::repository::ebuild", "", 7, [
   [
    ["self"]
   ]
   , ["ebuild"]
  ]]
  , [11, "clone", "grease::repository::package", "", 8, [
   [
    ["self"]
   ]
   , ["package"]
  ]]
  , [11, "clone", "grease::repository", "", 9, [
   [
    ["self"]
   ]
   , ["repository"]
  ]]
  , [11, "as_ref", "grease::repository::category", "", 4, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository::ebuild", "", 7, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository::package", "", 8, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "as_ref", "grease::repository", "", 9, [
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
  , [11, "fmt", "", "", 2, [
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
  , [11, "fmt", "grease::repository::category", "", 4, [
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
  , [11, "fmt", "", "", 5, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "", "", 2, [
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
  , [11, "fmt", "", "", 6, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository::ebuild", "", 7, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "grease::repository::package", "", 8, [
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
  , [11, "fmt", "", "", 9, [
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
  , [11, "name", "", "", 2, [
   [
    ["self"]
   ]
   , [
    ["option", ["str"]]
    , ["str"]
   ]
  ]]
  , [11, "cause", "", "", 2, [
   [
    ["self"]
   ]
   , [
    ["option", ["fail"]]
    , ["fail"]
   ]
  ]]
  , [11, "backtrace", "", "", 2, [
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
  , [4, "CategoryDirsError"]
  , [4, "CategoryMatcher"]
  , [3, "Category"]
  , [3, "CategoryFileIterator"]
  , [3, "CategoryDirsIterator"]
  , [3, "Ebuild"]
  , [3, "Package"]
  , [3, "Repository"]
 ]
};
initSearch(searchIndex);
addSearchOptions(searchIndex);
