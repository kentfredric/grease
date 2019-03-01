#[derive(Clone)]
/// A container for aspects of a Portage Version
pub struct Version {
    pv:  String,
    pr:  String,
    pvr: String,
}

impl Version {
    /// Construct a new Version struct
    pub fn new(pv: String, pr: String, pvr: String) -> Version { Version { pv, pr, pvr } }

    /// Return the packages version as per PMS' PV
    pub fn pv(&self) -> &str { &self.pv }

    /// Return the packages revision as per PMS' PR
    pub fn pr(&self) -> &str { &self.pr }

    /// Return the full version (perhaps with a -r suffix ) as per PMS' PVR
    pub fn pvr(&self) -> &str { &self.pvr }
}

/// Construct a Version object by parsing a full pvr string
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
