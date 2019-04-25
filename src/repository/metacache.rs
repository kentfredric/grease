//! An API for auto-generating and obtaining copies of metadata cache
use crate::repository::{Ebuild, Repository};
use ::crypto::{digest::Digest, md5::Md5};
use ::directories::ProjectDirs;
use ::lru::LruCache;
use ::std::{
    borrow::ToOwned,
    fmt,
    fs::{self, File},
    io::{ErrorKind, Read},
    os::unix::process::ExitStatusExt,
    panic,
    path::{Path, PathBuf},
    process::Command,
    result::Result::{Err, Ok},
    str,
    string::String,
};
use ::tempfile::{Builder, TempDir};

mod cacheentry;
mod md5cachedir;

use cacheentry::CacheEntry;
use md5cachedir::Md5CacheDir;

pub struct MetaDataCache {
    r:                    Repository,
    ebuild_md5_cache:     LruCache<String, String>,
    eclass_md5_cache:     LruCache<String, String>,
    ebuild_cache:         LruCache<String, CacheEntry>,
    ebuild_md5_cache_dir: Md5CacheDir,
    cache_dir:            PathBuf,
    temp_dir:             TempDir,
}

impl MetaDataCache {
    pub fn new(r: Repository) -> Self {
        let pd =
            ProjectDirs::from("io.github.kentfredric", "", "grease-util")
                .unwrap();

        let c = MetaDataCache {
            r:                    r.to_owned(),
            ebuild_md5_cache:     LruCache::new(500),
            eclass_md5_cache:     LruCache::new(500),
            ebuild_cache:         LruCache::new(100),
            ebuild_md5_cache_dir: Md5CacheDir::new(
                r.get_dir("metadata/md5-cache").unwrap(),
                None,
            ),
            cache_dir:            pd.cache_dir().join(r.name().unwrap()),
            temp_dir:             Builder::new()
                .prefix("grease-util-")
                .rand_bytes(7)
                .tempdir()
                .unwrap(),
        };
        c.ensure_cache_dir();
        c
    }

    pub(crate) fn add_fallback_caches(&mut self, dirs: Vec<PathBuf>) {
        self.ebuild_md5_cache_dir.add_children(dirs)
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

    fn md5_file(&self, path: &Path) -> String {
        let mut f = File::open(path).unwrap();
        let mut buf = [0; 8 * 1024];
        let mut md5 = Md5::new();
        while let Ok(len) = f.read(&mut buf[..]) {
            if len == 0 {
                break;
            }
            md5.input(&buf[..len]);
        }
        md5.result_str()
    }

    fn get_eclass_md5(&mut self, name: &str) -> &String {
        let my_name = name.to_owned();
        if !self.eclass_md5_cache.contains(&my_name) {
            let p = self.r.eclass_path(&my_name).unwrap();
            self.eclass_md5_cache.put(my_name.to_owned(), self.md5_file(&p));
        }
        self.eclass_md5_cache.get(&my_name).unwrap()
    }

    fn get_cache_for(&mut self, ebuild: Ebuild) -> Option<CacheEntry> {
        let cache_key = format!("{}/{}", ebuild.category(), ebuild.pf());

        match self.get_disk_cache_for(ebuild.to_owned()) {
            None => {
                self.generate_cache_for(ebuild.to_owned());
                self.get_disk_cache_for(ebuild)
            },
            other => other,
        }
    }

    fn cache_config(&self, name: &str, repo: &str) -> String {
        format!(
            "[DEFAULT]
             main-repo = {name}

             [{name}]
             location = {repo}
             sync-type = rsync
             syc-uri = rsync://invalid
             ",
            name = name,
            repo = repo,
        )
    }

    fn generate_cache_for(&mut self, ebuild: Ebuild) -> () {
        let repo_name = self.r.name().unwrap();
        let target = format!(
            "{category}/{package}",
            category = ebuild.category(),
            package = ebuild.pn()
        );
        let mut job = Command::new("egencache")
            .args(&[
                "--repo",
                &repo_name,
                "--repositories-configuration",
                &self.cache_config(
                    &repo_name,
                    self.r.path().to_str().unwrap(),
                ),
                "--cache-dir",
            ])
            .arg(self.cache_dir.as_os_str())
            .args(&["--tolerant", "--jobs", "3", "--update", &target])
            .spawn()
            .expect("Can't start egencache");

        let exit = job.wait().expect("Failed to wait for egencache");
        match exit.code() {
            Some(130) => panic!("egencache exited by sigint"),
            Some(0) => (),
            Some(s) => panic!("egencache exited with value {}", s),
            None => match exit.signal() {
                None => panic!("egencache killed by unknown signal"),
                Some(s) => panic!("egencache killed by signal {}", s),
            },
        }
    }

    fn get_disk_cache_for(&mut self, ebuild: Ebuild) -> Option<CacheEntry> {
        let cache_key = format!("{}/{}", ebuild.category(), ebuild.pf());
        let cache_leaves = self.ebuild_md5_cache_dir.get_iter(&cache_key);
        'leaf: for leaf in cache_leaves {
            let cache_entry = CacheEntry::read_from(&leaf);
            let md5 = self.md5_file(&ebuild.path());
            if md5 != cache_entry.md5() {
                continue;
            }
            for (eclass, emd5) in cache_entry.eclasses() {
                if md5 != *self.get_eclass_md5(&eclass) {
                    continue 'leaf;
                }
            }
            return Some(cache_entry);
        }
        return None;
    }
}
impl fmt::Debug for MetaDataCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Foo")
            .field("r", &self.r)
            .field("cache_dir", &self.cache_dir)
            .field("temp_dir", &self.temp_dir)
            .finish()
    }
}

#[test]
fn test_get_md5() {
    let eclasses = vec![
        "alternatives",
        "ant-tasks",
        "apache-2",
        "apache-module",
        "aspell-dict-r1",
        "autotools",
        "autotools-multilib",
        "autotools-utils",
        "base",
        "bash-completion-r1",
        "bazel",
        "bsdmk",
        "bzr",
        "cannadic",
        "cargo",
        "cdrom",
        "check-reqs",
        "chromium-2",
        "cmake-multilib",
        "cmake-utils",
        "common-lisp-3",
        "cron",
        "cuda",
        "cvs",
        "darcs",
        "db",
        "db-use",
        "depend.apache",
        "desktop",
        "distutils-r1",
        "dotnet",
        "eapi7-ver",
        "elisp-common",
        "elisp",
        "emboss-r2",
        "epatch",
        "epunt-cxx",
        "estack",
        "eutils",
        "fcaps",
        "fdo-mime",
        "findlib",
        "fixheadtails",
        "flag-o-matic",
        "font-ebdftopcf",
        "font",
        "fortran-2",
        "fox",
        "freebsd",
        "freedict",
        "games",
        "games-mods",
        "ghc-package",
        "git-2",
        "git-r3",
        "gkrellm-plugin",
        "gnome2",
        "gnome2-utils",
        "gnome.org",
        "gnome-python-common-r1",
        "gnuconfig",
        "gnustep-2",
        "gnustep-base",
        "golang-base",
        "golang-build",
        "golang-vcs",
        "golang-vcs-snapshot",
        "gstreamer",
        "haskell-cabal",
        "java-ant-2",
        "java-osgi",
        "java-pkg-2",
        "java-pkg-opt-2",
        "java-pkg-simple",
        "java-utils-2",
        "java-virtuals-2",
        "java-vm-2",
        "kde5",
        "kde5-functions",
        "kde5-meta-pkg",
        "kernel-2",
        "kodi-addon",
        "l10n",
        "latex-package",
        "leechcraft",
        "libretro-core",
        "libtool",
        "linux-info",
        "linux-mod",
        "llvm",
        "ltprune",
        "mate-desktop.org",
        "mate",
        "mercurial",
        "meson",
        "mono",
        "mono-env",
        "mount-boot",
        "mozconfig-v6.52",
        "mozconfig-v6.60",
        "mozcoreconf-v4",
        "mozcoreconf-v5",
        "mozcoreconf-v6",
        "mozextension",
        "mozlinguas-v2",
        "multibuild",
        "multilib-build",
        "multilib",
        "multilib-minimal",
        "multiprocessing",
        "myspell",
        "myspell-r2",
        "mysql-cmake",
        "mysql_fx",
        "mysql-multilib-r1",
        "mysql-v2",
        "netsurf",
        "ninja-utils",
        "nsplugins",
        "nvidia-driver",
        "oasis",
        "obs-download",
        "obs-service",
        "office-ext-r1",
        "opam",
        "openib",
        "out-of-source",
        "pam",
        "pax-utils",
        "perl-app",
        "perl-functions",
        "perl-module",
        "php-ext-pecl-r3",
        "php-ext-source-r2",
        "php-ext-source-r3",
        "php-pear-r2",
        "portability",
        "postgres",
        "postgres-multi",
        "prefix",
        "preserve-libs",
        "python-any-r1",
        "python-r1",
        "python-single-r1",
        "python-utils-r1",
        "qmail",
        "qmake-utils",
        "qt5-build",
        "readme.gentoo",
        "readme.gentoo-r1",
        "rebar",
        "ros-catkin",
        "rpm",
        "ruby-fakegem",
        "ruby-ng",
        "ruby-ng-gnome2",
        "ruby-single",
        "ruby-utils",
        "rust-toolchain",
        "s6",
        "savedconfig",
        "scons-utils",
        "selinux-policy-2",
        "sgml-catalog",
        "ssl-cert",
        "stardict",
        "subversion",
        "sword-module",
        "systemd",
        "texlive-common",
        "texlive-module",
        "tmpfiles",
        "toolchain-autoconf",
        "toolchain-binutils",
        "toolchain",
        "toolchain-funcs",
        "toolchain-glibc",
        "twisted-r1",
        "udev",
        "unpacker",
        "user",
        "vala",
        "vcs-clean",
        "vcs-snapshot",
        "vdr-plugin-2",
        "versionator",
        "vim-doc",
        "vim-plugin",
        "vim-spell",
        "virtualx",
        "waf-utils",
        "webapp",
        "wxwidgets",
        "xdg",
        "xdg-utils",
        "xemacs-elisp-common",
        "xemacs-elisp",
        "xemacs-packages",
        "xfconf",
        "xorg-2",
        "xorg-3",
    ];

    let r = Repository::new(Path::new("/usr/portage"));
    let mut mc = MetaDataCache::new(r);
    for i in 1..20 {
        for v in &eclasses {
            drop(mc.get_eclass_md5(v));
        }
    }
    drop(mc);
    panic!("done");
}
