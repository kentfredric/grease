#![allow(dead_code,non_snake_case)]

use ::std::{
    borrow::ToOwned,
    cmp::Ord,
    collections::HashMap,
    fmt::{self, Display},
    fs::File,
    io::{self, BufRead, BufReader},
    iter::Iterator,
    mem::drop,
    option::Option,
    panic,
    path::Path,
    result::Result::Ok,
    str,
    string::String,
    vec::Vec,
    write, writeln,
};

#[derive(Debug)]
pub(super) struct CacheEntry {
    values: HashMap<String, String>,
}

impl CacheEntry {
    fn new() -> Self { CacheEntry { values: HashMap::new() } }

    pub(super) fn read_from(p: &Path) -> Self {
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

    pub(super) fn eclasses(&self) -> Vec<(String, String)> {
        match self.raw__eclasses_() {
            None => Vec::new(),
            Some(s) => {
                let cvec: Vec<&str> = s.split('\u{0009}').collect();
                cvec.chunks(2)
                    .map(|e| (e[0].to_owned(), e[1].to_owned()))
                    .collect()
            },
        }
    }

    fn raw_homepage(&self) -> Option<&String> { self.values.get("HOMEPAGE") }

    fn raw_iuse(&self) -> Option<&String> { self.values.get("IUSE") }

    fn raw_keywords(&self) -> Option<&String> { self.values.get("KEYWORDS") }

    fn raw_license(&self) -> Option<&String> { self.values.get("LICENSE") }

    fn raw__md5_(&self) -> Option<&String> { self.values.get("_md5_") }

    pub(super) fn md5(&self) -> String {
        match self.raw__md5_() {
            None => "".to_owned(),
            Some(s) => s.to_owned(),
        }
    }

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
