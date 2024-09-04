
# CrabStructor v0.2.0-beta

## Init with literal

```rust
#[derive(Constructor, PartialEq, Debug)]
struct Example {
    #[init(10)]
    field: i32,
}

assert_eq!(Example::new(), Example {field: 10});
```

## Call new func

```rust
#[derive(Constructor, PartialEq, Debug)]
struct Example {
    #[new("string")]
    field: Arc<String>,
}

assert_eq!(Example::new(), Example {field: Arc::new("string".into())});
```

```rust
#[derive(Constructor, PartialEq, Debug)]
struct Example {
    #[new(arc_string: String)]
    field: Arc<String>,
}

assert_eq!(Example::new("string".to_string()), Example {field: Arc::new("string".into())});
```

# Alert

***This is dev version of lib***

## Supported types
1. All strings, which implement "Into" trait
2. All rust nums
3. Bool types
