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
    }
}
