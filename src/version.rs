use std::cell::RefCell;
use std::ffi::OsString;
use std::option::Option;

pub struct Version {
    version: OsString,
    pv: RefCell<Option<Option<String>>>,
    pr: RefCell<Option<Option<String>>>,
}

impl Version {
    pub fn new(version: OsString) -> Version { Version { version , pv: RefCell::new(None), pr: RefCell::new(None)} }
    pub fn pv(&self) -> Option<String> {
        if self.pv.borrow().is_none() {
            self.pv.replace(Some(self.pvr().map(|pvr| {
                let mut chunks: Vec<_> = pvr.split_terminator("-r").collect();
                if chunks.len() < 2 {
                    pvr
                } else {
                    match chunks.pop() {
                        None => pvr,
                        Some(rversion) => {
                            match rversion.parse::<u32>() {
                                Ok(_) => chunks.join("-"),
                                Err(_) => pvr,
                            }
                        },
                    }
                }
            })));
        }
        self.pv.borrow().to_owned().unwrap_or(None)
    }
    pub fn pr(&self) -> Option<String> {
        if self.pr.borrow().is_none() {
            self.pr.replace(Some(self.pvr().map(|pvr| {
                let mut chunks: Vec<_> = pvr.split_terminator("-r").collect();
                // This finds the trailing block of PVR which *might* be "-r" + a series of
                // digits if such a block exists, return the r-suffix, otherwise, perform no
                // changes and return PVR as PV
                if chunks.len() < 2 {
                    String::from("r0")
                } else {
                    match chunks.pop() {
                        None => String::from("r0"),
                        Some(rversion) => {
                            match rversion.parse::<u32>() {
                                Ok(_) => String::from("r".to_owned() + rversion),
                                Err(_) => String::from("r0"),
                            }
                        },
                    }
                }
            })));
        }
        self.pr.borrow().to_owned().unwrap_or(None)
    }
    pub fn pvr(&self) -> Option<String> { self.version.to_str().map(String::from) }
}
