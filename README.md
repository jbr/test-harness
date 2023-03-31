# test-harness

this proc macro wraps your tests with any function that accepts a
function with your test's signature. Your test function can accept any
number of arguments and return anything, as long as you call it with
the correct arguments in the harness.

## Example
```rust
use test_harness::test;

fn my_test_harness<F>(test: F)
where F: FnOnce(String) -> Result<(), &'static str> {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string).expect("test success");
}

#[test(harness = my_test_harness)]
fn my_test(random_string: String) -> Result<(), &'static str> {
    assert_eq!(string.len(), 10);
    Ok(())
}
```


This expands to the following, with no further macro magic

```rust
fn my_test_harness<F>(test: F)
where F: FnOnce(String) -> Result<(), &'static str> {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string).expect("test success");
}

#[test]
fn my_test() {
    fn my_test(random_string: String) -> Result<(), &'static str> {
        assert_eq!(string.len(), 10);
        Ok(())
    }
    my_test_harness(my_test);
}
```

## Async example

You can use this to set up an async runtime and spawn or block on the test.

```rust
use test_harness::test;

mod my_mod {
    pub fn set_up<F, Fut>(test: F)
    where
        F: FnOnce(&'static str) -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
    {
        futures_lite::future::block_on(test("hello")).unwrap()
    }
}

#[test(harness = my_mod::set_up)]
async fn my_test(s: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(s, "hello");
    Ok(())
}
```
