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
