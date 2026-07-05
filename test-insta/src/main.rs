/*!
A simple Rust program that demonstrates snapshot testing using the `insta` crate.

Follow these steps to run the tests and review the snapshots:

```bash
$ cargo test
$ cargo insta review
```

To add `insta` crate to your `Cargo.toml`, do the following:

```bash
$ cargo add insta --dev --features yaml
```

It enables the macro `insta::assert_yaml_snapshot!()`.
*/


fn main() {
    println!("Hello, world!");
}

pub fn generate_long_text() -> String {
    "This is a very long string that we want to test with snapshot testing.".repeat(10)
}

#[test]
fn test_huge_string() {
    insta::assert_snapshot!(generate_long_text());
}

#[test]
fn test_vec() {
    insta::assert_yaml_snapshot!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
