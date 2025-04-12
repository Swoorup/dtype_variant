use dtype_variant::{DType, build_dtype_tokens};

build_dtype_tokens!([I32, F32]);

pub trait DPrim: 'static {}
impl DPrim for i32 {}
impl DPrim for f32 {}

#[derive(DType, Clone, Debug)]
#[dtype(tokens = "self", matcher = "match_dprimtype")]
enum DPrimType {
    I32,
    F32,
}

#[derive(DType, Clone, Debug)]
#[dtype(constraint = "DPrim", tokens = "self", container = "Vec", matcher = "match_enum")]
enum DynChunk {
    I32(Vec<i32>),
    F32(Vec<f32>),
}

impl DynChunk {
    pub fn length(&self) -> usize {
        match_enum!(self, DynChunk<T, V>(inner) => { inner.len() })
    }

    pub fn prim_type(&self) -> DPrimType {
        match_enum!(self, DynChunk<T, V>(_inner, variant) => {
            variant.into()
        })
    }

    pub fn add(&self, dtype_variant: &DynChunk) -> DynChunk {
        match_enum!(self, DynChunk<T, V>(inner) => {
          let dtype_variant_inner = dtype_variant.downcast_ref::<V>().unwrap();
          let added = inner.iter().zip(dtype_variant_inner).map(|(a, b)| a + b).collect::<Vec<_>>();
          DynChunk::from(added)
        })
    }
}

fn main() {
    // Create and add DynChunks
    let chunk1 = DynChunk::from(vec![1, 2, 3]);
    let sum_chunk = chunk1.add(&DynChunk::from(vec![4, 5, 6]));
    println!("Sum of chunks: {:?}", sum_chunk); // Should print [5, 7, 9]
    println!("Length of sum_chunk: {}", sum_chunk.length());

    // Create and add floating point chunks
    let chunk2 = DynChunk::from(vec![1.0, 2.0, 3.0]);
    let float_sum = chunk2.add(&DynChunk::from(vec![0.1, 0.2, 0.3]));
    println!("Sum of float chunks: {:?}", float_sum); // Should print [1.1, 2.2, 3.3]
    println!("Length of float_sum: {}", float_sum.length());

    let primitive_type = chunk1.prim_type();
    println!("Primitive type of chunk1: {:?}", primitive_type);

    let primitive_type = chunk2.prim_type();
    println!("Primitive type of chunk2: {:?}", primitive_type);
}
