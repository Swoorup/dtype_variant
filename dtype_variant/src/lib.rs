#![allow(clippy::approx_constant)]

pub use dtype_variant_derive::{DType, build_dtype_tokens};

pub trait EnumVariantDowncast<VariantToken> {
    type Target;

    /// Consumes the enum and returns the target value if it matches the variant
    fn downcast(self) -> Option<Self::Target>;
}

pub trait EnumVariantDowncastRef<VariantToken> {
    type Target<'target> where Self: 'target;

    /// Returns a reference wrapper for the target field if the enum is the target variant
    fn downcast_ref(&self) -> Option<Self::Target<'_>>;
}

pub trait EnumVariantDowncastMut<VariantToken> {
    type Target<'target> where Self: 'target;

    /// Returns a mutable reference wrapper for the target field if the enum is the target variant
    fn downcast_mut(&mut self) -> Option<Self::Target<'_>>;
}

// Define the EnumVariantConstraint trait with Constraint parameter
pub trait EnumVariantConstraint<VariantToken> {
    type Constraint: 'static;
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Constraint: 'static {}

    impl Constraint for u16 {}
    impl Constraint for u32 {}
    impl Constraint for u64 {}

    build_dtype_tokens!([U16, U32, U64]);

    #[derive(Clone, Debug, Default, DType)]
    #[dtype(
        matcher = match_my_enum_variant,
        shared_variant_zst_path = self,
        constraint = Constraint
    )]
    pub enum MyEnumVariant {
        U16,
        U32,
        #[default]
        U64,
    }

    #[derive(Clone, Debug, DType, PartialEq, Eq)]
    #[dtype(
        matcher = match_my_enum,
        shared_variant_zst_path = self,
        constraint = Constraint,
        container = "Vec"
    )]
    enum MyEnum {
        U16(Vec<u16>),
        U32(Vec<u32>),
        U64(Vec<u64>),
    }

    impl MyEnum {
        fn from_default_variant(kind: MyEnumVariant) -> Self {
            match_my_enum_variant!(kind, MyEnumVariant<Variant>, MyEnum<Container, Constraint> => {
                vec![Constraint::default()].into()
            })
        }
    }

    #[test]
    fn test_simple_enum() {
        let a = MyEnumVariant::U16;
        let _b = MyEnumVariant::U32;
        match_my_enum_variant!(a, MyEnumVariant<VariantToken> => {
        });
    }

    #[test]
    fn test_end_to_end() {
        let x = MyEnum::from(vec![1_u16, 1, 2, 3, 5]);
        let bit_size = match_my_enum!(&x, MyEnum<T, VariantToken>(inner) => { inner.len() * T::BITS as usize });
        assert_eq!(bit_size, 80);
        let x = x.downcast::<U16Variant>().unwrap();
        assert_eq!(x[0], 1);
    }

    #[test]
    fn test_constraint() {
        let x = MyEnumVariant::U16;
        let my_enum = MyEnum::from_default_variant(x);
        assert_eq!(my_enum, MyEnum::U16(vec![0]));
    }

    #[test]
    fn test_token_based_downcast() {
        let x = MyEnum::from(vec![1_u16, 1, 2, 3, 5]);
        let first_element = x.downcast_ref::<U16Variant>().unwrap()[0];
        assert_eq!(first_element, 1_u16);
    }

    build_dtype_tokens!([I32, F32]);

    #[derive(Clone, Debug, DType)]
    #[dtype(matcher = match_dyn_enum, shared_variant_zst_path = self)]
    enum DynChunk {
        I32(i32),
        F32(f32),
    }

    #[test]
    fn test_dyn_chunk() {
        let x = DynChunk::from(42_i32);
        if let DynChunk::I32(value) = x {
            assert_eq!(value, 42);
        } else {
            panic!("Expected DynChunk::I32");
        }

        let mut y = DynChunk::from(3.14_f32);
        if let DynChunk::F32(value) = y {
            assert_eq!(value, 3.14);
        } else {
            panic!("Expected DynChunk::F32");
        }

        let downcasted: Option<&i32> = x.downcast_ref::<I32Variant>();
        assert_eq!(*downcasted.unwrap(), 42);

        let downcasted_mut: Option<&mut f32> = y.downcast_mut::<F32Variant>();
        *downcasted_mut.unwrap() = 2.71;
        if let DynChunk::F32(value) = y {
            assert_eq!(value, 2.71);
        }
    }

    #[test]
    fn test_match_dyn_enum_usage() {
        let x = DynChunk::from(42_i32);
        match_dyn_enum!(x, DynChunk<T, Token>(value) => {
            let str_repr = value.to_string();
            assert_eq!(str_repr, "42");
        });

        let y = DynChunk::from(3.14_f32);
        match_dyn_enum!(y, DynChunk<T, Token>(value) => {
            let str_repr = value.to_string();
            assert_eq!(str_repr, "3.14");
        });
    }

    build_dtype_tokens!([Int, Float, Str]); // Add tokens for MyData

    #[derive(DType, Debug, Clone, PartialEq)]
    #[dtype(shared_variant_zst_path = self)] // skip_from_impls is false by default
    #[dtype_grouped_matcher(name = match_by_category, grouping = [
        Numeric(Int | Float),
        Text(Str)
    ])]
    #[dtype_grouped_matcher(name = match_by_size, grouping = [Small(Int), Large(Float | Str)])]
    #[allow(dead_code)]
    enum MyData {
        Int(i32),
        Float(f64),
        Str(String),
    }

    #[test]
    fn test_grouped_matchers_mydata() {
        let int_val = MyData::Int(42);
        let float_val = MyData::Float(3.14);
        let str_val = MyData::Str("hello".to_string());

        // Test match_by_category
        let category_int = match_by_category!(int_val.clone(), {
            Numeric: MyData<T, Variant>(inner) => { format!("Numeric: {}", inner) },
            Text: MyData<T, Variant>(inner) => { format!("Text: {}", inner) },
        });
        let category_float = match_by_category!(float_val.clone(), {
            Numeric: MyData<T, Variant>(inner) => { format!("Numeric: {}", inner) },
            Text: MyData<T, Variant>(inner) => { format!("Text: {}", inner) },
        });
        let category_str = match_by_category!(str_val.clone(), { // Clone str_val as match consumes
            Numeric: MyData<T, Variant>(inner) => { format!("Numeric: {}", inner) },
            Text: MyData<T, Variant>(inner) => { format!("Text: {}", inner) },
        });

        assert_eq!(category_int, "Numeric: 42");
        assert_eq!(category_float, "Numeric: 3.14");
        assert_eq!(category_str, "Text: hello");

        // Test match_by_size
        let size_int = match_by_size!(&int_val, { // Use reference
            Small: MyData<T, Variant>(_inner) => { "Small" },
            Large: MyData<T, Variant>(_inner) => { "Large" },
        });
        let size_float = match_by_size!(&float_val, {
            Small: MyData<T, Variant>(_inner) => { "Small" },
            Large: MyData<T, Variant>(_inner) => { "Large" },
        });
        let size_str = match_by_size!(&str_val, {
            Small: MyData<T, Variant>(_inner) => { "Small" },
            Large: MyData<T, Variant>(_inner) => { "Large" },
        });

        assert_eq!(size_int, "Small");
        assert_eq!(size_float, "Large");
        assert_eq!(size_str, "Large");
    }

    build_dtype_tokens!([Person, Location, Score]); // Add tokens for struct variant test

    #[derive(DType, Debug, Clone, PartialEq)]
    #[dtype(matcher = match_struct_variant_data, shared_variant_zst_path = self)]
    #[allow(dead_code)]
    enum StructVariantData {
        Person { name: String, age: u32 },
        Location { lat: f64, lng: f64 },
        Score(i32),
    }

    #[test]
    fn test_struct_variants() {
        // Test struct variant creation
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let location_data = StructVariantData::Location {
            lat: 37.7749,
            lng: -122.4194,
        };
        
        let score_data = StructVariantData::Score(95);
        
        // Test owned downcasting (struct variants only support owned downcasting for now)
        if let Some(person_fields) = person_data.clone().downcast::<PersonVariant>() {
            assert_eq!(person_fields.name, "Alice");
            assert_eq!(person_fields.age, 30);
        } else {
            panic!("Failed to downcast person variant");
        }
        
        if let Some(location_fields) = location_data.clone().downcast::<LocationVariant>() {
            assert_eq!(location_fields.lat, 37.7749);
            assert_eq!(location_fields.lng, -122.4194);
        } else {
            panic!("Failed to downcast location variant");
        }
        
        if let Some(score) = score_data.downcast_ref::<ScoreVariant>() {
            assert_eq!(*score, 95);
        } else {
            panic!("Failed to downcast score variant");
        }
        
        // Test From implementations with struct types
        let person_struct = PersonFields { name: "Bob".to_string(), age: 25 };
        let data_from_struct = StructVariantData::from(person_struct);
        
        if let StructVariantData::Person { name, age } = data_from_struct {
            assert_eq!(name, "Bob");
            assert_eq!(age, 25);
        } else {
            panic!("Failed to create enum from struct");
        }
    }

    #[test]
    fn test_struct_variant_matcher_basic() {
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };

        // Test basic matcher functionality for struct variants - use simplest form
        let person_result = match_struct_variant_data!(person_data, StructVariantData<Token> => {
            "Person variant matched"
        });
        assert_eq!(person_result, "Person variant matched");
    }

    #[test]
    fn test_struct_variant_matcher_all_variants() {
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let location_data = StructVariantData::Location {
            lat: 37.7749,
            lng: -122.4194,
        };
        
        let score_data = StructVariantData::Score(95);

        // Test basic matcher functionality for all variants
        let person_result = match_struct_variant_data!(person_data, StructVariantData<Token> => {
            "Person variant"
        });
        assert_eq!(person_result, "Person variant");

        let location_result = match_struct_variant_data!(location_data, StructVariantData<Token> => {
            "Location variant"
        });
        assert_eq!(location_result, "Location variant");

        let score_result = match_struct_variant_data!(score_data, StructVariantData<Token> => {
            "Score variant"
        });
        assert_eq!(score_result, "Score variant");
    }

    #[test]
    fn test_struct_variant_matcher_with_references() {
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };

        // Test matcher with references
        let person_result = match_struct_variant_data!(&person_data, StructVariantData<Token> => {
            "Reference to person variant"
        });
        assert_eq!(person_result, "Reference to person variant");
    }

    #[test]
    fn test_struct_variant_reference_downcasting() {
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };
        
        let location_data = StructVariantData::Location {
            lat: 37.7749,
            lng: -122.4194,
        };

        // Test reference downcasting for struct variants - should now work!
        let person_ref = person_data.downcast_ref::<PersonVariant>();
        if let Some(person_fields_ref) = person_ref {
            assert_eq!(person_fields_ref.name, "Alice");
            assert_eq!(*person_fields_ref.age, 30);
        } else {
            panic!("Reference downcasting should work for struct variants now");
        }

        // Test reference downcasting for location
        let location_ref = location_data.downcast_ref::<LocationVariant>();
        if let Some(location_fields_ref) = location_ref {
            assert_eq!(*location_fields_ref.lat, 37.7749);
            assert_eq!(*location_fields_ref.lng, -122.4194);
        } else {
            panic!("Reference downcasting should work for struct variants now");
        }

        // Test mutable reference downcasting
        let mut person_data_mut = StructVariantData::Person {
            name: "Bob".to_string(),
            age: 25,
        };
        
        let person_mut_ref = person_data_mut.downcast_mut::<PersonVariant>();
        if let Some(person_fields_mut) = person_mut_ref {
            // We can modify the fields through the mutable references
            *person_fields_mut.age = 26;
            // Note: The original enum is not modified because we're working with a wrapper struct
            // This is expected behavior for this design
        } else {
            panic!("Mutable reference downcasting should work for struct variants now");
        }
    }

    #[test]
    fn test_struct_variant_reference_downcast_fail() {
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };

        // Test that downcasting to wrong type returns None
        let location_ref = person_data.downcast_ref::<LocationVariant>();
        assert!(location_ref.is_none());

        let mut person_data_clone = person_data.clone();
        let location_mut_ref = person_data_clone.downcast_mut::<LocationVariant>();
        assert!(location_mut_ref.is_none());
    }

    #[test]
    fn test_new_trait_structure() {
        // Test that the new three-trait structure works correctly
        let score_data = StructVariantData::Score(95);
        
        // Test each trait separately
        // Owned downcasting should work for tuple variants
        let score_owned = score_data.clone().downcast::<ScoreVariant>();
        assert_eq!(score_owned, Some(95));
        
        // Reference downcasting should work for tuple variants
        let score_ref = score_data.downcast_ref::<ScoreVariant>();
        assert_eq!(score_ref, Some(&95));
        
        // Mutable reference downcasting should work for tuple variants
        let mut score_data_mut = StructVariantData::Score(42);
        let score_mut_ref = score_data_mut.downcast_mut::<ScoreVariant>();
        assert_eq!(score_mut_ref, Some(&mut 42));
    }

    #[test]
    fn test_reference_struct_types() {
        // Test that the generated reference types work as expected
        let person_data = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };

        // Test that downcast_ref returns PersonRef<'_>
        let person_ref: Option<PersonRef<'_>> = person_data.downcast_ref::<PersonVariant>();
        assert!(person_ref.is_some());
        let person_ref = person_ref.unwrap();
        assert_eq!(person_ref.name, "Alice");
        assert_eq!(*person_ref.age, 30);

        // Test that downcast_mut returns PersonMut<'_>
        let mut person_data_mut = StructVariantData::Person {
            name: "Bob".to_string(),
            age: 25,
        };
        let person_mut: Option<PersonMut<'_>> = person_data_mut.downcast_mut::<PersonVariant>();
        assert!(person_mut.is_some());
        let person_mut = person_mut.unwrap();
        assert_eq!(person_mut.name, "Bob");
        *person_mut.age = 26; // Demonstrate mutable access
        assert_eq!(*person_mut.age, 26);
    }

    #[test]
    fn test_complete_reference_downcasting_functionality() {
        // Comprehensive test demonstrating the complete functionality
        
        // Test struct variants with reference downcasting
        let person = StructVariantData::Person {
            name: "Alice".to_string(),
            age: 30,
        };
        
        // Test owned downcasting (returns PersonFields)
        let person_owned = person.clone().downcast::<PersonVariant>().unwrap();
        assert_eq!(person_owned.name, "Alice");
        assert_eq!(person_owned.age, 30);
        
        // Test reference downcasting (returns PersonRef<'_>)
        let person_ref = person.downcast_ref::<PersonVariant>().unwrap();
        assert_eq!(person_ref.name, "Alice");
        assert_eq!(*person_ref.age, 30);
        
        // Test mutable reference downcasting (returns PersonMut<'_>)
        let mut person_mut = StructVariantData::Person {
            name: "Bob".to_string(),
            age: 25,
        };
        let person_mut_ref = person_mut.downcast_mut::<PersonVariant>().unwrap();
        assert_eq!(person_mut_ref.name, "Bob");
        *person_mut_ref.age = 26;
        assert_eq!(*person_mut_ref.age, 26);
        
        // Test tuple variants still work
        let score = StructVariantData::Score(95);
        assert_eq!(score.clone().downcast::<ScoreVariant>(), Some(95));
        assert_eq!(score.downcast_ref::<ScoreVariant>(), Some(&95));
        
        let mut score_mut = StructVariantData::Score(42);
        assert_eq!(score_mut.downcast_mut::<ScoreVariant>(), Some(&mut 42));
        
        // Test wrong type downcasting returns None
        assert!(person.downcast_ref::<LocationVariant>().is_none());
        assert!(person.downcast_ref::<ScoreVariant>().is_none());
    }

}
