![Crates.io Version](https://img.shields.io/crates/v/crabstructor?style=flat-square&label=crabstructor)

# Auto constructor derive for structures

Simple constructor generator for named structures

```rust
#[derive(Constructor, Eq, PartialEq, Debug)]
#[constructor(
    field1 = r#"String::from("test")"#
)]
struct Example {
    field1: String,
    field2: i32
}

assert_eq!(Example::new(2), Example {field1: "test".to_string(), field2: 2});
```



