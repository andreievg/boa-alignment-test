use boa_engine::{Context, Source};

#[no_mangle]
pub extern "C" fn test_boa_alignment() -> i32 {
    println!("Starting Boa alignment test...");

    let mut context = Context::default();

    // Test cases that might trigger alignment issues
    let test_cases = vec![
        // Basic object creation
        "let obj = {}; obj",
        // Property assignment (might trigger NaN boxing)
        "let obj = {}; for (let i = 0; i < 100; i++) { obj[`prop${i}`] = i; } obj",
        // Number operations that use NaN boxing
        "let nums = []; for (let i = 0; i < 100; i++) { nums.push(Math.random()); } nums",
        // String operations
        "let strs = []; for (let i = 0; i < 50; i++) { strs.push(`string${i}`); } strs",
        // Mixed types (most likely to trigger alignment issues)
        "let mixed = [1, 'test', {}, [], true, null, undefined]; mixed",
    ];

    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Running test case {}: {}", i + 1, test_case);

        let source = Source::from_bytes(test_case);
        match context.eval(source) {
            Ok(result) => {
                println!("✓ Test case {} passed: {:?}", i + 1, result);
            }
            Err(e) => {
                println!("✗ Test case {} failed: {}", i + 1, e);
                return 1;
            }
        }
    }

    println!("All test cases passed!");
    0
}

// Additional stress test
#[no_mangle]
pub extern "C" fn stress_test_boa() -> i32 {
    println!("Starting Boa stress test...");

    let mut context = Context::default();

    // Stress test that creates many objects and values
    let source = Source::from_bytes(
        r#"
        let results = [];
        for (let i = 0; i < 1000; i++) {
            let obj = {
                id: i,
                name: `object_${i}`,
                value: Math.random(),
                nested: {
                    array: [i, i+1, i+2],
                    bool: i % 2 === 0
                }
            };
            results.push(obj);
        }
        results.length;
    "#,
    );

    match context.eval(source) {
        Ok(result) => {
            println!("✓ Stress test passed: {:?}", result);
            0
        }
        Err(e) => {
            println!("✗ Stress test failed: {}", e);
            1
        }
    }
}
