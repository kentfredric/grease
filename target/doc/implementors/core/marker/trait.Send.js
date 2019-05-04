(function() {
 var implementors = {};
 implementors["grease"] = [{
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"grease/repository/struct.Repository.html\" title=\"struct grease::repository::Repository\">Repository</a>"
  , synthetic: true
  , types: ["grease::repository::Repository"]
 }, {
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"grease/repository/enum.RepositoryError.html\" title=\"enum grease::repository::RepositoryError\">RepositoryError</a>"
  , synthetic: true
  , types: ["grease::repository::RepositoryError"]
 }, {
  text: "impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"grease/repository/category/struct.Category.html\" title=\"struct grease::repository::category::Category\">Category</a>"
  , synthetic: true
  , types: ["grease::repository::category::Category"]
 }, ];

 if (window.register_implementors) {
  window.register_implementors(implementors);
 } else {
  window.pending_implementors = implementors;
 }

})()
