# Rust Result and Option

Rust has enum types that represent values with a small set of possible forms.

## Option

`Option<T>` represents a value that may or may not exist.

It has two variants:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
let value: Option<i32> = Some(5);
let missing: Option<i32> = None;
```

`Some(value)` means a value exists. `None` means there is no value.

## Result

`Result<T, E>` represents an operation that can either succeed or fail.

It has two variants:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` is the success value type. `E` is the error value type.

Example:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(String::from("cannot divide by zero"))
    }
}
```

This function returns:

- `Ok(value)` when the operation succeeds
- `Err(error)` when the operation fails

The explicit form also works:

```rust
Result::Ok(value)
Result::Err(error)
```

But Rust code usually uses the shorter `Ok(...)` and `Err(...)` forms.

## Terminology

`Option` and `Result` are enums.

`Some`, `None`, `Ok`, and `Err` are enum variants.

More specifically:

- `Option<T>` is a generic enum type
- `Result<T, E>` is a generic enum type
- `T` and `E` are generic type parameters

## Other Common Enums

Rust has many enums besides `Option` and `Result`, including:

- `Ordering`: `Less`, `Equal`, `Greater`
- `Poll<T>`: `Ready(T)`, `Pending`
- `ControlFlow<B, C>`: `Break(B)`, `Continue(C)`
- `IpAddr`: `V4(...)`, `V6(...)`

You can also define your own enums:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

`Option` and `Result` are not special syntax. They are regular enums that are used heavily throughout Rust.
