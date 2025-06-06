use boa_engine::{Context, Source};

#[no_mangle]
pub extern "C" fn test_boa_alignment() -> i32 {
    // Minimal reproduction case
    let mut context = Context::default();

    // Try operations that might trigger the alignment issue
    let source = Source::from_bytes(
        "
        let obj = {};
        for (let i = 0; i < 1000; i++) {
            obj[`prop${i}`] = i;
        }
        obj
    ",
    );

    match context.eval(source) {
        Ok(_) => {
            println!("Test passed");
            0
        }
        Err(e) => {
            println!("Test failed: {}", e);
            1
        }
    }
}
