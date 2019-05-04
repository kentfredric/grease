(function() {
 var implementors = {};
 implementors["grease"] = [{
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"grease/repository/category/struct.Category.html\" title=\"struct grease::repository::category::Category\">Category</a>"
  , synthetic: false
  , types: ["grease::repository::category::Category"]
 }, {
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"grease/repository/enum.RepositoryError.html\" title=\"enum grease::repository::RepositoryError\">RepositoryError</a>"
  , synthetic: false
  , types: ["grease::repository::RepositoryError"]
 }, {
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"grease/repository/struct.Repository.html\" title=\"struct grease::repository::Repository\">Repository</a>"
  , synthetic: false
  , types: ["grease::repository::Repository"]
 }, ];

 if (window.register_implementors) {
  window.register_implementors(implementors);
 } else {
  window.pending_implementors = implementors;
 }

})()
