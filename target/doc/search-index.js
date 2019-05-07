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
  , [11, "new", "", "Construct a new Repository trait object", 1, [
   [
    ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 1, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "name", "", "Extract this repositories name from its profiles dir", 1, [
   [
    ["self"]
   ]
   , [
    ["result", ["string", "repositoryerror"]]
    , ["string"]
    , ["repositoryerror"]
   ]
  ]]
  , [11, "to_owned", "", "", 1, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 1, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 1, [
   []
   , ["u"]
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
  , [11, "clone", "", "", 1, [
   [
    ["self"]
   ]
   , ["repository"]
  ]]
  , [11, "as_ref", "", "", 1, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "fmt", "", "", 0, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
  , [11, "fmt", "", "", 0, [
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
  , [11, "name", "", "", 0, [
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
  , [3, "Repository"]
 ]
};
initSearch(searchIndex);
addSearchOptions(searchIndex);
