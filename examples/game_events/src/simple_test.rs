// Backup simple test that should definitely work
use dtype_variant::{DType, build_dtype_tokens};

build_dtype_tokens!([A, B]);

#[derive(DType, Debug)]
#[dtype(shared_variant_zst_path = self)]
enum Simple {
    A(i32),
    B(String),
}

fn test_simple() {
    let a = Simple::A(42);
    let b = Simple::B("hello".to_string());
    
    println!("Created: {:?}, {:?}", a, b);
    
    if let Some(val) = a.downcast_ref::<AVariant>() {
        println!("A contains: {}", val);
    }
    
    if let Some(val) = b.downcast_ref::<BVariant>() {
        println!("B contains: {}", val);
    }
}