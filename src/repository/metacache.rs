//! An API for auto-generating and obtaining copies of metadata cache
use crate::repository::Repository;
use directories::ProjectDirs;
use lru::LruCache;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs::{self, File},
    io::{self, BufRead, BufReader, ErrorKind},
    path::{Path, PathBuf},
    str,
};
use tempfile::{Builder, TempDir};

pub struct MetaDataCache {
    r:                Repository,
    ebuild_md5_cache: LruCache<String, String>,
    eclass_md5_cache: LruCache<String, String>,
    ebuild_cache:     LruCache<String, CacheEntry>,
    cache_dir:        PathBuf,
    temp_dir:         TempDir,
}
#[derive(Debug)]
struct CacheEntry {
    values: HashMap<String, String>,
}

impl CacheEntry {
    fn new() -> Self { CacheEntry { values: HashMap::new() } }

    fn read_from(p: &Path) -> Self {
        let mut h: HashMap<String, String> = HashMap::new();
        let br = BufReader::new(File::open(p).unwrap());
        for line in br.lines() {
            let l = line.unwrap();
            let items: Vec<&str> = l.splitn(2, '=').collect();
            if items.is_empty() {
                panic!("Invalid empty line in {:?}", p);
            }
            if items.len() < 2 {
                panic!("Too few tokens in {:?} on line: {:?}", p, l);
            }
            drop(h.insert(items[0].to_owned(), items[1].to_owned()));
        }
        Self { values: h }
    }

    fn raw_defined_phases(&self) -> Option<&String> {
        self.values.get("DEFINED_PHASES")
    }

    fn raw_depend(&self) -> Option<&String> { self.values.get("DEPEND") }

    fn raw_description(&self) -> Option<&String> {
        self.values.get("DESCRIPTION")
    }

    fn raw_eapi(&self) -> Option<&String> { self.values.get("EAPI") }

    fn raw__eclasses_(&self) -> Option<&String> {
        self.values.get("_eclasses_")
    }

    fn raw_homepage(&self) -> Option<&String> { self.values.get("HOMEPAGE") }

    fn raw_iuse(&self) -> Option<&String> { self.values.get("IUSE") }

    fn raw_keywords(&self) -> Option<&String> { self.values.get("KEYWORDS") }

    fn raw_license(&self) -> Option<&String> { self.values.get("LICENSE") }

    fn raw__md5_(&self) -> Option<&String> { self.values.get("_md5_") }

    fn raw_pdepend(&self) -> Option<&String> { self.values.get("PDEPEND") }

    fn raw_properties(&self) -> Option<&String> {
        self.values.get("PROPERTIES")
    }

    fn raw_rdepend(&self) -> Option<&String> { self.values.get("RDEPEND") }

    fn raw_required_use(&self) -> Option<&String> {
        self.values.get("REQUIRED_USE")
    }

    fn raw_restrict(&self) -> Option<&String> { self.values.get("RESTRICT") }

    fn raw_slot(&self) -> Option<&String> { self.values.get("SLOT") }

    fn raw_src_uri(&self) -> Option<&String> { self.values.get("SRC_URI") }

    fn write_to(&self, dest: &mut dyn io::Write) -> io::Result<()> {
        let mut nodes: Vec<(&String, &String)> = self.values.iter().collect();
        nodes.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));
        for (key, value) in nodes {
            writeln!(dest, "{}={}", key, value)?;
        }
        Ok(())
    }
}

impl Display for CacheEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b: Vec<u8> = Vec::new();
        self.write_to(&mut b).unwrap();
        write!(f, "{}", str::from_utf8(&b).unwrap())
    }
}

impl fmt::Debug for MetaDataCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Foo")
            .field("r", &self.r)
            .field("cache_dir", &self.cache_dir)
            .field("temp_dir", &self.temp_dir)
            .finish()
    }
}

impl MetaDataCache {
    pub fn new(r: Repository) -> Self {
        let pd =
            ProjectDirs::from("io.github.kentfredric", "", "grease-util")
                .unwrap();

        let c = MetaDataCache {
            r:                r.to_owned(),
            ebuild_md5_cache: LruCache::new(100),
            eclass_md5_cache: LruCache::new(100),
            ebuild_cache:     LruCache::new(100),
            cache_dir:        pd.cache_dir().join(r.name().unwrap()),
            temp_dir:         Builder::new()
                .prefix("grease-util-")
                .rand_bytes(7)
                .tempdir()
                .unwrap(),
        };
        c.ensure_cache_dir();
        c
    }

    fn ensure_cache_dir(&self) {
        match self.cache_dir.metadata() {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    fs::create_dir_all(&self.cache_dir).unwrap();
                },
                _ => panic!("Cache directory is not usable: {}", e),
            },
            Ok(m) => {
                if !m.is_dir() {
                    panic!(
                        "Cache directory {:?} exists but is not a dir",
                        &self.cache_dir
                    );
                }
            },
        }
    }
}
