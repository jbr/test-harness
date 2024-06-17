#![allow(clippy::assertions_on_constants)]
type StrResult = Result<(), &'static str>;

mod returning_unit {
    use test_harness::test;
    fn harness(test: impl Fn(bool)) {
        test(true)
    }

    #[test(harness)]
    fn test_elided_harness_name(pass: bool) {
        assert!(pass);
    }

    #[test(harness = harness)]
    fn test_harness_name_is_harness(pass: bool) {
        assert!(pass);
    }

    fn custom_harness_name(test: impl Fn(bool)) {
        test(true)
    }

    #[test(harness = custom_harness_name)]
    fn test_custom_harness_name(pass: bool) {
        assert!(pass);
    }

    #[test]
    fn test_basic() {
        assert!(true);
    }
}

mod with_result {
    use super::StrResult;
    use test_harness::test;

    fn harness(test: impl Fn(bool) -> StrResult) -> StrResult {
        test(true)
    }

    #[test(harness)]
    fn test_elided_harness_name(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }

    #[test(harness = harness)]
    fn test_harness_name_is_harness(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }

    fn custom_harness_name(test: impl Fn(bool) -> StrResult) -> StrResult {
        test(true)
    }

    #[test(harness = custom_harness_name)]
    fn test_custom_harness_name(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }
}

mod optional_result {
    use super::StrResult;
    use test_harness::test;

    fn harness<T: std::process::Termination>(test: impl Fn(bool) -> T) -> T {
        test(true)
    }

    #[test(harness)]
    fn test_elided_harness_name_returning_result(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }

    #[test(harness)]
    fn test_elided_harness_name_returning_unit(pass: bool) {
        assert!(pass);
    }

    #[test(harness = harness)]
    fn test_harness_name_is_harness_returning_result(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }

    #[test(harness = harness)]
    fn test_harness_name_is_harness_returning_unit(pass: bool) {
        assert!(pass);
    }

    fn custom_harness_name<T: std::process::Termination>(test: impl Fn(bool) -> T) -> T {
        test(true)
    }

    #[test(harness = custom_harness_name)]
    fn test_custom_harness_name_returning_result(pass: bool) -> StrResult {
        assert!(pass);
        Ok(())
    }

    #[test(harness = custom_harness_name)]
    fn test_custom_harness_name_returning_unit(pass: bool) {
        assert!(pass);
    }
}

mod passthrough {
    use super::StrResult;
    use test_harness::test;

    #[test]
    fn test_returning_result() -> StrResult {
        assert!(true);
        Ok(())
    }

    #[test]
    fn test_returning_unit() {
        assert!(true);
    }
}

mod attributes {
    use test_harness::test;
    fn harness(test: impl FnOnce()) {
        test()
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        panic!()
    }

    #[test]
    #[deny(unsafe_code)]
    fn inner_attributes_without_harness() {
        #![should_panic]
        #![allow(unsafe_code, unused_unsafe)]
        assert_eq!(unsafe { 1 + 1 }, 2);
        panic!();
    }

    #[test(harness)]
    #[ignore]
    fn ignored_test_with_elided_harness() {
        panic!()
    }

    #[test(harness = harness)]
    #[ignore]
    fn ignored_test_with_named_harness() {
        panic!()
    }

    #[test(harness = harness)]
    #[deny(unsafe_code)]
    fn inner_attrs() {
        #![should_panic]
        #![allow(unsafe_code, unused_unsafe)] // this wouldn't compile if #inner was skipped
        assert_eq!(unsafe { 1 + 1 }, 2);
        panic!();
    }

    #[test(harness = harness)]
    #[allow(unsafe_code, unused_unsafe)] // this wouldn't compile if #outer was skipped
    #[should_panic]
    fn outer_attrs() {
        assert_eq!(unsafe { 1 + 1 }, 2);
        panic!();
    }
}

pub mod visibility {
    //! I'm not quite sure why anyone would need this, but it can't hurt to propagate fn visibility
    mod some_test {
        fn harness(test: impl FnOnce()) {
            test()
        }
        #[test_harness::test(harness)]
        pub(super) fn some_test() {}
    }

    pub fn x() {
        some_test::some_test(); // this would fail to compile if #vis was omitted
    }
}
