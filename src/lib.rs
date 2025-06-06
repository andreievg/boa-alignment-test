use boa_engine::{Context, Source};

#[no_mangle]
pub extern "C" fn test_boa_alignment() -> i32 {
    test();
    1
}

fn test() {
    let mut context = Context::default();
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
        }
        Err(e) => {
            println!("Test failed: {}", e);
        }
    }
}
