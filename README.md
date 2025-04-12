# dtype_variant

A Rust derive macro for creating type-safe enum variants with shared type tokens across multiple enums. This enables synchronized variant types and powerful downcasting capabilities between related enums.

## Features

- 🔄 Share and synchronize variant types across multiple enums
- ✨ Type-safe downcasting of enum variants using token types
- 🔒 Compile-time validation of variant types
- 📦 Optional container type support (e.g., Vec, Box)
- 🔍 Constraint trait implementation for variant types
- 🎯 Flexible pattern matching through generated macros
- 🛠️ Convenient From implementations for variant types

For example, you can define primitive type tokens that are shared between a type enum and its corresponding data container:

```rust
#[derive(DType)]
enum PrimType {    // Type enum
    I32,
    F32,
}

#[derive(DType)]
enum DataChunk {   // Data container enum
    I32(Vec<i32>),
    F32(Vec<f32>),
}
```

This ensures that operations between related enums remain type-safe and synchronized at compile time.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dtype_variant = "0.0.1"
```

## Usage

```rust
use dtype_variant::DType;

// First, define your token types (usually generated)
mod tokens {
    pub struct FloatVariant;
    pub struct IntegerVariant;
}

#[derive(DType)]
#[dtype(
    tokens = "tokens",          // Required: Path to token types
    container = "Vec",          // Optional: Container type for variants
    constraint = "ToString",    // Optional: Trait constraint for variant types
    matcher = "match_number"    // Optional: Name for the generated matcher macro
)]
enum Number {
    Float(Vec<f64>),
    Integer(Vec<i32>),
}

fn main() {
    let num = Number::Float(vec![1.0, 2.0, 3.0]);

    // Type-safe downcasting
    if let Some(floats) = num.downcast_ref::<tokens::FloatVariant>() {
        println!("Found floats: {:?}", floats);
    }

    // Pattern matching using generated macro
    match_number!(num, Number<T, Token>(value) => {
        println!("Value: {:?}", value);
    });
}
```

## Features

### Type-safe Downcasting

Access variant data with compile-time type checking:

```rust
let num = Number::Float(vec![1.0, 2.0]);

// Safe downcasting methods
let float_ref: Option<&Vec<f64>> = num.downcast_ref::<tokens::FloatVariant>();
let float_mut: Option<&mut Vec<f64>> = num.downcast_mut::<tokens::FloatVariant>();
let owned_float: Option<Vec<f64>> = num.downcast::<tokens::FloatVariant>();
```

### Container Types

Optionally wrap variant data in container types:

```rust
#[derive(DType)]
#[dtype(tokens = "tokens", container = "Vec")]
enum Data {
    Numbers(Vec<i32>),
    Text(Vec<String>),
}
```

### Trait Constraints

Enforce trait bounds on variant types:

```rust
#[derive(DType)]
#[dtype(tokens = "tokens", constraint = "std::fmt::Display")]
enum Printable {
    Text(String),
    Number(i32),
}
```

### Pattern Matching

Generate ergonomic pattern matching macros:

```rust
match_data!(value, Data<T, Token>(inner) => {
    println!("Got value of type {} with data: {:?}",
             std::any::type_name::<T>(), inner);
});
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

This project was inspired by [dtype_dispatch](https://github.com/pcodec/pcodec/tree/main/dtype_dispatch), which provides similar enum variant type dispatch functionality.
