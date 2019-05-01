mod repository {

    use grease::repository::Repository;

    #[test]
    fn new() {
        let r = Repository::new("");
        let _p = r.path();
    }

}
