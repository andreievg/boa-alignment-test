use boa_engine::{Context, Source};

/// Test basic Boa operations that might trigger alignment issues
pub fn test_boa_alignment() -> i32 {
    println!("Starting Boa alignment test...");

    let mut context = Context::default();

    let test_cases = vec![
        // Basic object creation
        ("Basic object", "let obj = {}; obj"),
        // Property assignment (might trigger NaN boxing issues)
        (
            "Property assignment",
            "let obj = {}; for (let i = 0; i < 100; i++) { obj[`prop${i}`] = i; } obj",
        ),
        // Number operations that use NaN boxing
        (
            "Number operations",
            "let nums = []; for (let i = 0; i < 100; i++) { nums.push(Math.random()); } nums",
        ),
        // String operations
        (
            "String operations",
            "let strs = []; for (let i = 0; i < 50; i++) { strs.push(`string${i}`); } strs",
        ),
        // Mixed types (most likely to trigger alignment issues)
        (
            "Mixed types",
            "let mixed = [1, 'test', {}, [], true, null, undefined]; mixed",
        ),
        // Function calls and closures
        (
            "Functions",
            "let fn1 = () => 42; let fn2 = (x) => x * 2; [fn1(), fn2(21)]",
        ),
        // Complex nested structures
        (
            "Nested structures",
            r#"
            let complex = {
                level1: {
                    level2: {
                        level3: {
                            numbers: [1, 2, 3, 4, 5],
                            strings: ["a", "b", "c"],
                            mixed: [1, "two", 3.0, true, null]
                        }
                    }
                }
            };
            complex
        "#,
        ),
    ];

    for (i, (name, test_case)) in test_cases.iter().enumerate() {
        println!("Running test case {}: {}", i + 1, name);

        let source = Source::from_bytes(test_case);
        match context.eval(source) {
            Ok(_) => {
                println!("✓ Test case {} ({}) passed", i + 1, name);
            }
            Err(e) => {
                println!("✗ Test case {} ({}) failed: {}", i + 1, name, e);
                return 1;
            }
        }
    }

    println!("All basic test cases passed!");
    0
}

/// Stress test that creates many objects and values
pub fn stress_test_boa() -> i32 {
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
                    bool: i % 2 === 0,
                    func: () => i * 2
                }
            };
            results.push(obj);
            
            // Occasionally trigger garbage collection scenarios
            if (i % 100 === 0) {
                let temp = [];
                for (let j = 0; j < 50; j++) {
                    temp.push({data: j, more: `temp_${j}`});
                }
                // temp goes out of scope
            }
        }
        results.length;
    "#,
    );

    match context.eval(source) {
        Ok(result) => {
            println!("✓ Stress test passed: created {:?} objects", result);
            0
        }
        Err(e) => {
            println!("✗ Stress test failed: {}", e);
            1
        }
    }
}

/// Test rapid object creation and destruction
pub fn memory_pressure_test() -> i32 {
    println!("Starting memory pressure test...");

    let mut context = Context::default();

    let source = Source::from_bytes(
        r#"
        // Create memory pressure by rapidly creating and discarding objects
        for (let round = 0; round < 10; round++) {
            let batch = [];
            for (let i = 0; i < 500; i++) {
                batch.push({
                    id: i + round * 500,
                    data: new Array(100).fill(0).map((_, idx) => idx),
                    timestamp: Date.now(),
                    random: Math.random()
                });
            }
            // batch goes out of scope, should be eligible for GC
        }
        "Memory pressure test completed"
    "#,
    );

    match context.eval(source) {
        Ok(_) => {
            println!("✓ Memory pressure test passed");
            0
        }
        Err(e) => {
            println!("✗ Memory pressure test failed: {}", e);
            1
        }
    }
}

/// Export C functions for FFI if needed later
#[no_mangle]
pub extern "C" fn c_test_boa_alignment() -> i32 {
    test_boa_alignment()
}

#[no_mangle]
pub extern "C" fn c_stress_test_boa() -> i32 {
    stress_test_boa()
}

#[no_mangle]
pub extern "C" fn c_memory_pressure_test() -> i32 {
    memory_pressure_test()
}
