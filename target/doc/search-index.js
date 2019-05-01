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
  , [11, "new", "", "Construct a new Repository trait object", 0, [
   [
    ["p"]
   ]
   , ["self"]
  ]]
  , [11, "path", "", "Returns the path to this repository", 0, [
   [
    ["self"]
   ]
   , ["pathbuf"]
  ]]
  , [11, "to_owned", "", "", 0, [
   [
    ["self"]
   ]
   , ["t"]
  ]]
  , [11, "clone_into", "", "", 0, [
   [
    ["t"]
    , ["self"]
   ]
  ]]
  , [11, "into", "", "", 0, [
   []
   , ["u"]
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
  , [11, "clone", "", "", 0, [
   [
    ["self"]
   ]
   , ["repository"]
  ]]
  , [11, "fmt", "", "", 0, [
   [
    ["self"]
    , ["formatter"]
   ]
   , ["result"]
  ]]
 ]
 , "p": [
  [3, "Repository"]
 ]
};
initSearch(searchIndex);
addSearchOptions(searchIndex);
