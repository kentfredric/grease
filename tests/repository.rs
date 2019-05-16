mod util;

mod repository {
    use crate::util::repos;
    use grease::repository::Repository;

    #[test]
    fn new() {
        let r = Repository::new("");
        let _p = r.path();
    }

    #[test]
    fn path() {
        let test_root = repos("basic").unwrap();
        let r = Repository::new(&test_root);

        let p = r.path();
        assert_eq!(test_root, p);
    }

    #[test]
    fn name() {
        let test_root = repos("basic").unwrap();
        let r = Repository::new(&test_root);

        let n = r.name().unwrap();
        assert_eq!(n, "grease-test");
    }

    #[test]
    fn as_ref_path() {
        let test_root = repos("basic").unwrap();
        let r = Repository::new(&test_root);

        let p = r.as_ref();
        assert_eq!(&test_root, p);
    }

    #[test]
    fn into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Repository::new(&test_root);

        let p: PathBuf = r.into();
        assert_eq!(test_root, p);
    }

    #[test]
    fn borrow_into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Repository::new(&test_root);

        let p: PathBuf = (&r).into();
        assert_eq!(test_root, p);
        // Check is borrowed this far
        drop(r);
    }

}

mod category {
    mod basic {
        use crate::util::repos;
        use grease::repository::Category;

        #[test]
        fn new() {
            let test_root = repos("basic").unwrap();
            let _r = Category::new(&test_root, "dev-perl");
        }

        #[test]
        fn path() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "dev-perl");

            assert_eq!(test_root.join("dev-perl"), r.path());
        }

        #[test]
        fn name() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "dev-perl");

            assert_eq!("dev-perl", r.name());
        }

        #[test]
        fn as_ref_path() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "dev-perl");

            let p = r.as_ref();
            assert_eq!(&test_root.join("dev-perl"), p);
        }
        #[test]
        fn into_pathbuf() {
            use std::path::PathBuf;
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "dev-perl");

            let p: PathBuf = r.into();
            assert_eq!(test_root.join("dev-perl"), p);
        }

        #[test]
        fn borrow_into_pathbuf() {
            use std::path::PathBuf;
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "dev-perl");

            let p: PathBuf = (&r).into();
            assert_eq!(test_root.join("dev-perl"), p);
            // Check is borrowed this far
            drop(r);
        }
    }

    mod noncat {
        use crate::util::repos;
        use grease::repository::Category;

        #[test]
        fn new() {
            let test_root = repos("basic").unwrap();
            let _r = Category::new(&test_root, "profiles");
        }

        #[test]
        fn path() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "profiles");

            assert_eq!(test_root.join("profiles"), r.path());
        }

        #[test]
        fn name() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "profiles");

            assert_eq!("profiles", r.name());
        }

    }

    mod nonexisting {
        use crate::util::repos;
        use grease::repository::Category;

        #[test]
        fn new() {
            let test_root = repos("basic").unwrap();
            let _r = Category::new(&test_root, "I-do-not-exist");
        }

        #[test]
        fn path() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "I-do-not-exist");

            assert_eq!(test_root.join("I-do-not-exist"), r.path());
        }

        #[test]
        fn name() {
            let test_root = repos("basic").unwrap();
            let r = Category::new(&test_root, "I-do-not-exist");

            assert_eq!("I-do-not-exist", r.name());
        }
    }
}

mod package {
    use crate::util::repos;
    use grease::repository::Package;

    #[test]
    fn new() {
        let test_root = repos("basic").unwrap();
        let _r = Package::new(&test_root, "dev-perl", "example");
    }

    #[test]
    fn path() {
        let test_root = repos("basic").unwrap();
        let r = Package::new(&test_root, "dev-perl", "example");

        assert_eq!(test_root.join("dev-perl").join("example"), r.path());
    }

    #[test]
    fn name() {
        let test_root = repos("basic").unwrap();
        let r = Package::new(&test_root, "dev-perl", "example");

        assert_eq!("dev-perl/example", r.name());
    }

    #[test]
    fn as_ref_pathbuf() {
        let test_root = repos("basic").unwrap();
        let r = Package::new(&test_root, "dev-perl", "example");

        let p = r.as_ref();
        assert_eq!(&test_root.join("dev-perl").join("example"), p);
    }

    #[test]
    fn into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Package::new(&test_root, "dev-perl", "example");

        let p: PathBuf = r.into();
        assert_eq!(test_root.join("dev-perl/example"), p);
    }

    #[test]
    fn borrow_into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Package::new(&test_root, "dev-perl", "example");

        let p: PathBuf = (&r).into();
        assert_eq!(test_root.join("dev-perl/example"), p);
        // Check is borrowed this far
        drop(r);
    }
}

mod ebuild {
    use crate::util::repos;
    use grease::repository::Ebuild;

    #[test]
    fn new() {
        let test_root = repos("basic").unwrap();
        let _r = Ebuild::new(
            &test_root,
            "dev-perl",
            "example",
            "example-0.1.0.ebuild",
        );
    }

    #[test]
    fn path() {
        let test_root = repos("basic").unwrap();
        let r = Ebuild::new(
            &test_root,
            "dev-perl",
            "example",
            "example-0.1.0.ebuild",
        );

        assert_eq!(
            test_root.join("dev-perl/example/example-0.1.0.ebuild"),
            r.path()
        );
    }

    #[test]
    fn as_ref_pathbuf() {
        let test_root = repos("basic").unwrap();
        let r = Ebuild::new(
            &test_root,
            "dev-perl",
            "example",
            "example-0.1.0.ebuild",
        );
        let p = r.as_ref();
        assert_eq!(
            &test_root.join("dev-perl/example/example-0.1.0.ebuild"),
            p
        );
    }

    #[test]
    fn into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Ebuild::new(
            &test_root,
            "dev-perl",
            "example",
            "example-0.1.0.ebuild",
        );

        let p: PathBuf = r.into();
        assert_eq!(
            test_root.join("dev-perl/example/example-0.1.0.ebuild"),
            p
        );
    }

    #[test]
    fn borrow_into_pathbuf() {
        use std::path::PathBuf;
        let test_root = repos("basic").unwrap();
        let r = Ebuild::new(
            &test_root,
            "dev-perl",
            "example",
            "example-0.1.0.ebuild",
        );

        let p: PathBuf = (&r).into();
        assert_eq!(
            test_root.join("dev-perl/example/example-0.1.0.ebuild"),
            p
        );
        // Check is borrowed this far
        drop(r);
    }

}

mod category_file_iterator {
    use crate::util::repos;
    use grease::repository::{category::CategoryFileIterator, Category};

    #[test]
    fn basic() {
        let test_root = repos("basic").unwrap();
        let i = CategoryFileIterator::for_file(
            &test_root,
            &test_root.join("profiles/categories"),
        );
        let results: Vec<Category> = i
            .unwrap()
            .filter_map(|item| match item {
                Ok(x) => Some(x),
                Err(e) => panic!("{:?}", e),
            })
            .collect();
        assert_eq!(results, vec![Category::new(&test_root, "dev-perl")]);
    }
}
