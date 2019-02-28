#[derive(Clone)]
pub struct Version {
    pv:  String,
    pr:  String,
    pvr: String,
}

impl Version {
    pub fn new(pv: String, pr: String, pvr: String) -> Version { Version { pv, pr, pvr } }

    pub fn pv(&self) -> &str { &self.pv }

    pub fn pr(&self) -> &str { &self.pr }

    pub fn pvr(&self) -> &str { &self.pvr }
}

pub fn parse(version: &str) -> Version {
    let vx = version.to_string();
    let mut v_chunks: Vec<_> = vx.split_terminator("-r").collect();
    let tail = v_chunks.pop();
    if !v_chunks.is_empty()
        && tail.is_some()
        && tail.expect("no value returned from string iterator").parse::<u32>().is_ok()
    {
        let prefix = v_chunks.join("-");
        let suffix = "r".to_owned() + tail.unwrap();
        Version::new(prefix.to_owned(), suffix.to_owned(), prefix + "-" + &suffix)
    } else {
        Version::new(vx.to_owned(), String::from("r0"), vx.to_owned())
    }
}
