
# CrabStructor v0.2.0-alpha

```rust
#[derive(Constructor, Eq, PartialEq, Debug)]
struct Example {
    #[init(10)]
    field: i32,
}

assert_eq!(Example::new(), Example {field: 10});
```

# Alert

***This is not dev version of lib***

## Supported types
1. All strings, which implement Into trait
2. All rust nums
3. Bool types

# Todo
1. Write trait for easy support of any type