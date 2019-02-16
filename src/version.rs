use std::ffi::OsString;

pub struct Version {
    version: OsString,
    pv: String,
    pr: String,
}

impl Version {
    pub fn new(version: OsString) -> Version {
        let v_string = version.to_str().map(String::from).expect(
            "Can't decode OsString \
             to UTF8",
        );
        let mut v_chunks: Vec<_> = v_string.split_terminator("-r").collect();
        let tail = v_chunks.pop();

        if v_chunks.len() > 1 && tail.is_some() &&
            tail.expect("no value returned from string iterator")
                .parse::<u32>()
                .is_ok()
        {
            Version {
                version,
                pv: v_string,
                pr: String::from("r0"),
            }
        } else {
            Version {
                version,
                pv: v_chunks.join("-"),
                pr: String::from("r".to_owned() + tail.unwrap()),
            }
        }
    }
}
