
# dtype_variant

A Rust derive macro for creating type-safe enum variants with shared type tokens across multiple enums. This enables synchronized variant types and powerful downcasting capabilities between related enums.

## Features

- 🔄 Share and synchronize variant types across multiple enums
- ✨ Type-safe downcasting of enum variants using token types
- 🔒 Compile-time validation of variant types
- 📦 Optional container type support (e.g., Vec, Box)
- 🔍 Constraint trait implementation for variant types
- 🎯 Powerful pattern matching through generated macros
- 🛠️ Convenient From implementations for variant types
- 🔀 Grouped variant matching for matching related variants.

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
use dtype_variant::{DType, build_dtype_tokens};

// Generate token types for the variants
build_dtype_tokens!([Integer, Float, Complex]);

#[derive(DType)]
#[dtype(tokens_path = self, container = Vec)]
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
    if let Some(floats) = data.downcast_ref::<FloatVariant>() {
        println!("Processing float data: {:?}", floats);
    }
}

// Or use the generated pattern matching macro
match_numeric_data!(data, NumericData<T, Token>(values) => {
    println!("Processing {} data: {:?}",
             std::any::type_name::<T>(), values);
});
```

The crate especially shines when you have multiple related enums that need to stay in sync:

```rust
// Generate tokens for all variants
build_dtype_tokens!([Integer, Float, Complex]);

#[derive(DType)]
#[dtype(tokens_path = self)]
enum NumericType {  // Type enum
    Integer,
    Float,
    Complex,
}

#[derive(DType)]
#[dtype(tokens_path = self)]
enum NumericStats {  // Stats enum
    Integer(MinMaxStats<i64>),
    Float(MinMaxStats<f64>),
    Complex(ComplexStats),
}

#[derive(DType)]
#[dtype(tokens_path = self, container = Vec)]
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
use dtype_variant::{DType, build_dtype_tokens};

// Generate token types with the macro
build_dtype_tokens!([Float, Integer]);

#[derive(DType)]
#[dtype(
    tokens_path = self,          // Use tokens in current scope
    container = Vec,             // Optional: Container type for variants
    constraint = ToString,       // Optional: Trait constraint for variant types
    matcher = match_number       // Optional: Name for the generated matcher macro
)]
enum Number {
    Float(Vec<f64>),
    Integer(Vec<i32>),
}

fn main() {
    let num = Number::Float(vec![1.0, 2.0, 3.0]);

    // Type-safe downcasting
    if let Some(floats) = num.downcast_ref::<FloatVariant>() {
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
let float_ref: Option<&Vec<f64>> = num.downcast_ref::<FloatVariant>();
let float_mut: Option<&mut Vec<f64>> = num.downcast_mut::<FloatVariant>();
let owned_float: Option<Vec<f64>> = num.downcast::<FloatVariant>();
```

### Container Types

Optionally wrap variant data in container types:

```rust
build_dtype_tokens!([Numbers, Text]);

#[derive(DType)]
#[dtype(tokens_path = self, container = Vec)]
enum Data {
    Numbers(Vec<i32>),
    Text(Vec<String>),
}
```

### The Power of Generated Matcher Macros

One of `dtype_variant`'s most powerful features is its generated matcher macros, which provide capabilities beyond standard Rust pattern matching:

```rust
build_dtype_tokens!([Int, Float, Str]);

#[derive(DType)]
#[dtype(tokens_path = self)]
// Group variants by their logical category
#[dtype_grouped_matcher(name = match_by_category, grouping = [
    Numeric([Int, Float]),
    Text([Str])
])]
// Group variants by their memory footprint
#[dtype_grouped_matcher(name = match_by_size, grouping = [
    Small([Int]),
    Large([Float, Str])
])]
enum MyData {
    Int(i32),
    Float(f64),
    Str(String),
}

// Access actual type parameters in patterns
let data = MyData::Float(3.14);
match_my_data!(data, MyData<T, Token>(value) => {
    // This branch handles all variants
    // T is inferred as f64, i32 or String, Token as FloatVariant or IntVariant or StrVariant
    println!("Type: {}, Value: {}", std::any::type_name::<T>(), value);
});

// Match against logical groups of variants
let result = match_by_category!(data, {
    Numeric: MyData<T, Variant>(value) => {
        // This branch handles BOTH Int and Float variants
        // T is the actual type (either i32 or f64)
        format!("Processing numeric value: {}", value)
    },
    Text: MyData<T, Variant>(value) => {
        format!("Processing text: {}", value)
    }
});

// Or match by size characteristics
let size_class = match_by_size!(data, {
    Small: MyData<T, Variant>(_) => "Small data type",
    Large: MyData<T, Variant>(_) => "Large data type",
});
```

These matcher macros provide:

1. **Type Parameters in Patterns**: Access to the actual types of each variant
2. **Grouped Variant Matching**: Handle sets of variants together by logical categories
3. **Token Types in Patterns**: Full access to both the data type and token type
4. **Automatic Container Handling**: Seamless handling of container types

### Trait Constraints

Enforce trait bounds on variant types:

```rust
build_dtype_tokens!([Float, Integer]);

#[derive(DType)]
#[dtype(tokens_path = self, constraint = Display)]
enum FormattableNumber {
    Float(f64),
    Integer(i32),
}

// The constraint ensures all variant types implement Display
fn format_number(num: &FormattableNumber) -> String {
    match_formattable_number!(num, FormattableNumber<T, Token>(value) => {
        // We can now safely call .to_string() on any variant's value
        format!("Formatted number: {}", value.to_string())
    })
}
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

This project was inspired by [dtype_dispatch](https://github.com/pcodec/pcodec/tree/main/dtype_dispatch), which provides similar enum variant type dispatch functionality.
