use boa_engine::{Context, Source};

/// Test basic Boa operations that might trigger alignment issues
pub fn test_boa_alignment() -> i32 {
    println!("Starting Boa alignment test...");

    let mut context = Context::default();
    let source = Source::from_bytes("let obj = {}; obj");
    context.eval(source).unwrap();

    println!("Boa alignment test completed successfully.");
    1
}

/// Export C functions for FFI if needed later
#[no_mangle]
pub extern "C" fn c_test_boa_alignment() -> i32 {
    test_boa_alignment()
}
