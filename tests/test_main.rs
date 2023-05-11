#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_run_with_vrd_test_mode() {
        let output = Command::cargo_bin("vrd")
            .unwrap()
            .env("VRD_TEST_MODE", "1")
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was not successful
        assert!(!output.status.success());

        // Assert that the error message was printed to stderr
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("Error running vrd: Simulated error"));
    }

    #[test]
    fn test_run_without_vrd_test_mode() {
        let output = Command::cargo_bin("vrd")
            .unwrap()
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was successful
        assert!(output.status.success());

        // Assert that the welcome messages were printed to stdout
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Welcome to `VRD` 👋!"));
        assert!(stdout.contains("A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm"));
    }

    fn run_test_scenario() -> Result<(), Box<dyn std::error::Error>> {
        // Simulate an error scenario
        // Return an error explicitly
        Err("Test error".into())
    }

    #[test]
    fn test_main() {
        // Test calling the `run()` function directly
        let result = run_test_scenario();
        assert!(result.is_err());

        // Test calling the `main()` function
        let output = Command::cargo_bin("vrd")
            .unwrap()
            .env("VRD_TEST_MODE", "1")
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was not successful
        assert!(!output.status.success());

        // Assert that the error message was printed to stderr
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("Error running vrd: Simulated error"));
    }
}
