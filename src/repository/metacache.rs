use crate::repository::Repository;
use directories::ProjectDirs;
use std::{collections::HashMap, fs};

pub struct MetaDataCache {
    r:     Repository,
    cache: HashMap<String, ()>,
    dirs:  ProjectDirs,
}

impl MetaDataCache {
    pub fn new(r: Repository) -> Self {
        let c = MetaDataCache {
            r,
            cache: HashMap::new(),
            dirs: ProjectDirs::from(
                "io.github.kentfredric",
                "",
                "grease-util",
            )
            .unwrap(),
        };
        c.ensure_cache_dir();
        c
    }

    fn ensure_cache_dir(&self) {
        let cache_dir = self.dirs.cache_dir();
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir).unwrap();
        } else if !cache_dir.is_dir() {
            panic!("Cache directory {:?} exists but is not a dir", cache_dir);
        }
    }
}
