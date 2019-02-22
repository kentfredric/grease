#[derive(Clone)]
pub struct Version {
    pv: String,
    pr: String,
}

impl Version {
    pub fn new(pv: String, pr: String) -> Version { Version { pv, pr } }

    pub fn pv(&self) -> &str { &self.pv }

    pub fn pr(&self) -> &str { &self.pr }

    pub fn pvr(&self) -> String { self.pv().to_owned() + "-" + self.pr() }
}

pub fn parse(version: &str) -> Version {
    let vx = version.to_string();
    let mut v_chunks: Vec<_> = vx.split_terminator("-r").collect();
    let tail = v_chunks.pop();
    if !v_chunks.is_empty()
        && tail.is_some()
        && tail.expect("no value returned from string iterator").parse::<u32>().is_ok()
    {
        Version::new(v_chunks.join("-"), String::from("r".to_owned() + tail.unwrap()))
    } else {
        Version::new(vx.to_owned(), String::from("r0"))
    }
}
