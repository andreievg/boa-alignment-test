extern "C" {
    fn test_boa_alignment() -> i32;
    fn stress_test_boa() -> i32;
}

fn main() {
    println!("Boa Alignment Test on Android ARM64");
    println!("====================================");

    // Run basic alignment test
    let result1 = unsafe { test_boa_alignment() };
    if result1 != 0 {
        println!("Basic test failed!");
        std::process::exit(result1);
    }

    // Run stress test
    let result2 = unsafe { stress_test_boa() };
    if result2 != 0 {
        println!("Stress test failed!");
        std::process::exit(result2);
    }

    println!("All tests completed successfully!");
}
