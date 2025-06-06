use boa_alignment_test::{memory_pressure_test, stress_test_boa, test_boa_alignment};

fn main() {
    println!("Boa Alignment Test on Android ARM64");
    println!("====================================");
    println!("Testing pointer alignment issues in Boa JavaScript engine");
    println!();

    // Set up panic handler to get better error info
    std::panic::set_hook(Box::new(|panic_info| {
        println!("PANIC: {}", panic_info);
        if let Some(location) = panic_info.location() {
            println!(
                "Location: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
    }));

    let mut total_tests = 0;
    let mut failed_tests = 0;

    // Run basic alignment test
    println!("🔍 Running basic alignment tests...");
    total_tests += 1;
    let result1 = test_boa_alignment();
    if result1 != 0 {
        println!("❌ Basic alignment test failed!");
        failed_tests += 1;
    } else {
        println!("✅ Basic alignment test passed!");
    }
    println!();

    // Run stress test
    println!("🔍 Running stress test...");
    total_tests += 1;
    let result2 = stress_test_boa();
    if result2 != 0 {
        println!("❌ Stress test failed!");
        failed_tests += 1;
    } else {
        println!("✅ Stress test passed!");
    }
    println!();

    // Run memory pressure test
    println!("🔍 Running memory pressure test...");
    total_tests += 1;
    let result3 = memory_pressure_test();
    if result3 != 0 {
        println!("❌ Memory pressure test failed!");
        failed_tests += 1;
    } else {
        println!("✅ Memory pressure test passed!");
    }
    println!();

    // Summary
    println!("====================================");
    println!("Test Summary:");
    println!("  Total tests: {}", total_tests);
    println!("  Passed: {}", total_tests - failed_tests);
    println!("  Failed: {}", failed_tests);

    if failed_tests > 0 {
        println!("❌ Some tests failed - this may indicate the alignment issue!");
        std::process::exit(1);
    } else {
        println!("✅ All tests passed!");
    }
}
