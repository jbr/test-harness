# test-harness

this proc macro wraps your tests with any function that accepts a
function with your test's signature. Your test function can accept any
number of arguments and return anything, as long as you call it with
the correct arguments in the harness.

## Example
```rust
use test_harness::test;

fn my_test_harness(test: impl FnOnce(String)) {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string)
}

#[test(harness = my_test_harness)]
fn my_test(random_string: String) {
    assert_eq!(string.len(), 10);
}
```


This expands to the following, with no further macro magic

```rust
fn my_test_harness<F>(test: impl FnOnce(String)) {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string)
}

#[test]
fn my_test() -> impl std::process::Termination {
    fn my_test(random_string: String) -> Result<(), &'static str> {
        assert_eq!(string.len(), 10);
        Ok(())
    }
    my_test_harness(my_test)
}
```

## Returning Result
```rust
use test_harness::test;

fn my_test_harness<F>(test: F) -> Result<(), &'static str>
where F: FnOnce(String) -> Result<(), &'static str> {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string)
}

#[test(harness = my_test_harness)]
fn my_test(random_string: String) -> Result<(), &'static str> {
    assert_eq!(string.len(), 10);
    Ok(())
}
```


This expands to the following, with no further macro magic

```rust
fn my_test_harness<F>(test: F) -> Result<(), &'static str>
where F: FnOnce(String) -> Result<(), &'static str> {
    let string = std::iter::repeat_with(fastrand::alphanumeric).take(10).collect();
    test(string)
}

#[test]
fn my_test() -> impl std::process::Termination {
    fn my_test(random_string: String) -> Result<(), &'static str> {
        assert_eq!(string.len(), 10);
        Ok(())
    }
    my_test_harness(my_test)
}
```


## Async example

You can use this to set up an async runtime and spawn or block on the test.

```rust
use test_harness::test;

mod my_mod {
    pub fn set_up<F, Fut>(test: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&'static str) -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static,
    {
        futures_lite::future::block_on(test("hello"))
    }
}

#[test(harness = my_mod::set_up)]
async fn my_test(s: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(s, "hello");
    Ok(())
}
```


## Eliding harness name

If you name your harness `harness`, you can elide the harness name, like so:

```rust
use test_harness::test;

pub fn harness<F, Fut, Out>(test: F) -> Out
where
    F: FnOnce(&'static str) -> Fut,
    Fut: std::future::Future<Output = Out> + Send + 'static,
    Out: std::process::Termination
{
    futures_lite::future::block_on(test("hello"))
}


#[test(harness)]
async fn test_one(s: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(s, "hello");
    Ok(())
}

#[test(harness)]
async fn test_two(s: &'static str) {
    assert_eq!(s, "hello");
}

```




## Drop down to standard #[test]

If this macro is used without any additional arguments, it works identically to the built-in `#[test]` macro.

```rust
use test_harness::test;
#[test]
fn normal_test() {
    assert!(true);
}
```

