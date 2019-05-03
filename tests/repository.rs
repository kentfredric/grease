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
}
