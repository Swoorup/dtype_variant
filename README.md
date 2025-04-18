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

## Why?

Let's say you're building a data processing pipeline where you need to handle different numeric types. Without `dtype_variant`, you might start with something like this:

```rust
// Define types that your system can handle
enum NumericType {
    Integer,
    Float,
    Complex,
}

// Store actual data
enum NumericData {
    Integer(Vec<i64>),
    Float(Vec<f64>),
    Complex(Vec<Complex64>),
}

// Processing functions
impl NumericData {
    fn get_type(&self) -> NumericType {
        match self {
            NumericData::Integer(_) => NumericType::Integer,
            NumericData::Float(_) => NumericType::Float,
            NumericData::Complex(_) => NumericType::Complex,
        }
    }

    fn as_float_vec(&self) -> Option<&Vec<f64>> {
        match self {
            NumericData::Float(v) => Some(v),
            _ => None
        }
    }

    fn as_integer_vec(&self) -> Option<&Vec<i64>> {
        match self {
            NumericData::Integer(v) => Some(v),
            _ => None
        }
    }

    fn as_complex_vec(&self) -> Option<&Vec<Complex64>> {
        match self {
            NumericData::Complex(v) => Some(v),
            _ => None
        }
    }
}
```

This approach has several problems:

1. **Type Safety**: There's no compile-time guarantee that `NumericType` and `NumericData` variants stay in sync
2. **Boilerplate**: You need to write conversion methods for each type
3. **Extensibility**: Adding a new numeric type requires changes in multiple places
4. **Error-prone**: Easy to forget updating one enum when modifying the other

With `dtype_variant`, this becomes:

```rust
use dtype_variant::DType;

mod tokens {
    pub struct IntegerVariant;
    pub struct FloatVariant;
    pub struct ComplexVariant;
}

#[derive(DType)]
#[dtype(tokens_path = tokens, container = Vec)]
enum NumericData {
    Integer(Vec<i64>),
    Float(Vec<f64>),
    Complex(Vec<Complex64>),
}
```

Now you get:

1. **Type Safety**: Downcasting is handled through token types at compile time
2. **Zero Boilerplate**: Generic downcasting methods are automatically implemented
3. **Easy Extension**: Just add a new variant and its token type
4. **Pattern Matching**: Generated macros for ergonomic handling

```rust
fn process_data(data: &NumericData) {
    // Type-safe downcasting with zero boilerplate
    if let Some(floats) = data.downcast_ref::<tokens::FloatVariant>() {
        println!("Processing float data: {:?}", floats);
    }
}

// Or use the generated pattern matching macro
match_numeric!(data, NumericData<T, Token>(values) => {
    println!("Processing {} data: {:?}",
             std::any::type_name::<T>(), values);
});
```

The crate especially shines when you have multiple related enums that need to stay in sync:

```rust
#[derive(DType)]
#[dtype(tokens_path = tokens)]
enum NumericType {  // Type enum
    Integer,
    Float,
    Complex,
}

#[derive(DType)]
#[dtype(tokens_path = tokens)]
enum NumericStats {  // Stats enum
    Integer(MinMaxStats<i64>),
    Float(MinMaxStats<f64>),
    Complex(ComplexStats),
}

#[derive(DType)]
#[dtype(tokens_path = tokens, container = Vec)]
enum NumericData {  // Data enum
    Integer(Vec<i64>),
    Float(Vec<f64>),
    Complex(Vec<Complex64>),
}
```

All these enums share the same token types, ensuring they stay in sync and can safely interact with each other through the type system.

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
    tokens_path = tokens,          // Required: Path to token types
    container = Vec,          // Optional: Container type for variants
    constraint = ToString,    // Optional: Trait constraint for variant types
    matcher = match_number    // Optional: Name for the generated matcher macro
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
#[dtype(tokens_path = tokens, container = Vec)]
enum Data {
    Numbers(Vec<i32>),
    Text(Vec<String>),
}
```

### Trait Constraints

Enforce trait bounds on variant types:

```rust
#[derive(DType)]
#[dtype(tokens_path = tokens, constraint = std::fmt::Display)]
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
