use crate::category::Category;

pub fn exists(cat: &Category) -> bool { cat.path().exists() }

pub fn legal_name(cat: &Category) -> bool {
    match cat.category.as_str() {
        "metadata" | "profiles" | "eclass" | ".git" | "distfiles" | "packages" | "scripts" => false,
        _ => true,
    }
}

pub fn legal(cat: &Category) -> bool { legal_name(&cat) && cat.path().is_dir() }

pub fn nonempty(cat: &Category) -> bool { cat.packages().unwrap().any(|x| x.is_ok()) }
