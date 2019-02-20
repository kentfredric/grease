use crate::category::Category;

/// A filter that returns if a given category exists or not
pub fn exists(cat: &Category) -> bool { cat.path().exists() }

/// A filter that determines if a category has a legal name or not
pub fn legal_name(cat: &Category) -> bool {
    match cat.category.as_str() {
        "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => false,
        _ => true,
    }
}

/// A filter that determines if a category is "legal" or not
///
/// This means the category has both a legal name, and its path is a directory
pub fn legal(cat: &Category) -> bool { legal_name(&cat) && cat.path().is_dir() }

/// A filter that determines if a category has children
///
/// This is a perfomance hit because it has to invoke readdir on the category
/// and begins package discovery, but returns true as soon as readdir yelids a package
pub fn non_empty(cat: &Category) -> bool { cat.packages().unwrap().any(|x| x.is_ok()) }
