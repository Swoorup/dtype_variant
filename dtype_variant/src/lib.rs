#![allow(clippy::approx_constant)]

pub use dtype_variant_derive::{DType, build_dtype_tokens};

pub trait EnumVariantDowncast<VariantToken> {
    type Target;

    /// Returns a reference to the target field if the enum is the target variant
    fn downcast_ref(&self) -> Option<&Self::Target>;
    fn downcast_mut(&mut self) -> Option<&mut Self::Target>;
    fn downcast(self) -> Option<Self::Target>;
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

    #[derive(Clone, Debug, DType)]
    #[dtype(
        matcher = "test_match_enum",
        tokens = "self",
        constraint = "Constraint",
        container = "Vec"
    )]
    enum MyEnum {
        U16(Vec<u16>),
        U32(Vec<u32>),
        U64(Vec<u64>),
    }
    #[derive(Clone, Debug, Default, DType)]
    #[dtype(matcher = "test_match_enum_variant", tokens = "self", constraint = "Constraint")]
    pub enum MyEnumVariant {
        U16,
        U32,
        #[default]
        U64,
    }

    #[test]
    fn test_simple_enum() {
        let a = MyEnumVariant::U16;
        let _b = MyEnumVariant::U32;
        test_match_enum_variant!(a, MyEnumVariant<T, VariantToken>(inner) => {
        });
    }

    #[test]
    fn test_end_to_end() {
        let x = MyEnum::from(vec![1_u16, 1, 2, 3, 5]);
        let bit_size = test_match_enum!(&x, MyEnum<T, VariantToken>(inner) => { inner.len() * T::BITS as usize });
        assert_eq!(bit_size, 80);
        let x = x.downcast::<U16Variant>().unwrap();
        assert_eq!(x[0], 1);
    }

    #[test]
    fn test_token_based_downcast() {
        let x = MyEnum::from(vec![1_u16, 1, 2, 3, 5]);
        let first_element = x.downcast_ref::<U16Variant>().unwrap()[0];
        assert_eq!(first_element, 1_u16);
    }

    build_dtype_tokens!([I32, F32]);

    #[derive(Clone, Debug, DType)]
    #[dtype(matcher = "match_dyn_enum", tokens = "self")]
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
}
