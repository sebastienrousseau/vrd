// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use dtt::DateTime;
    use rlg::{log_format::LogFormat, log_level::LogLevel};
    use std::panic;
    use uuid::Uuid;
    use vrd::{create_log_entry, log_entry_async};

    // Logging tests
    /// Tests the creation of a log entry with specific details and verifies that the generated log entry contains the expected information.
    #[test]
    fn test_create_log_entry() {
        // Arrange
        let uuid = "test-uuid";
        let iso = "2023-06-10T12:34:56Z";
        let level = LogLevel::INFO;
        let message = "Test log message";

        // Act
        let log_entry = create_log_entry(uuid, iso, level, message);

        // Assert
        assert_eq!(log_entry.session_id, uuid);
        assert_eq!(log_entry.time, iso);
        assert_eq!(log_entry.level.to_string(), "INFO");
        assert_eq!(log_entry.component, "VRD");
        assert_eq!(log_entry.description, message);
        assert_eq!(log_entry.format, LogFormat::JSON);
    }

    /// Tests the creation of log entries with different log levels.
    #[test]
    fn test_create_log_entry_different_levels() {
        let uuid = "test-uuid";
        let iso = "2023-06-10T12:34:56Z";
        let message = "Test log message";

        for level in &[
            LogLevel::DEBUG,
            LogLevel::INFO,
            LogLevel::WARN,
            LogLevel::ERROR,
        ] {
            let log_entry =
                create_log_entry(uuid, iso, *level, message);
            assert_eq!(log_entry.level, *level);
        }
    }

    // Asynchronous logging tests
    /// Tests the asynchronous logging of a log entry to ensure it completes successfully.
    #[tokio::test]
    async fn test_log_entry_async() {
        // Arrange
        let uuid = "test-uuid";
        let iso = "2023-06-10T12:34:56Z";
        let level = LogLevel::INFO;
        let message = "Test log message";

        let log_entry = create_log_entry(uuid, iso, level, message);

        // Act
        let result = log_entry_async(log_entry).await;

        // Assert
        // Assert that the log entry was logged successfully
        assert!(result.is_ok());
    }

    /// Tests asynchronous logging using the Tokio runtime.
    #[tokio::test]
    async fn test_async_logging_with_runtime() {
        // Arrange
        let date = DateTime::new();
        let uuid = Uuid::new_v4().to_string();
        let log_entry = create_log_entry(
            &uuid,
            &date.iso_8601,
            LogLevel::INFO,
            "Test async logging",
        );

        // Act
        let result = log_entry_async(log_entry).await;

        // Assert
        assert!(result.is_ok());
    }

    // UUID tests
    /// Tests the generation of multiple UUIDs to ensure they are unique and non-empty.
    #[test]
    fn test_uuid_generation() {
        // Act
        // Test generating multiple UUIDs
        let uuid1 = Uuid::new_v4().to_string();
        let uuid2 = Uuid::new_v4().to_string();

        // Assert
        // Ensure UUIDs are generated successfully and are unique
        assert!(!uuid1.is_empty());
        assert!(!uuid2.is_empty());
        assert_ne!(uuid1, uuid2);
    }

    // DateTime tests
    /// Tests the creation of a DateTime object and verifies the format of the generated timestamp.
    #[test]
    fn test_datetime_creation() {
        let date = DateTime::new();
        assert!(!date.iso_8601.is_empty());
        assert!(date.iso_8601.len() >= 20); // Basic check for ISO 8601 format length
    }

    // Tokio runtime tests
    /// Tests the creation of a Tokio runtime to ensure it initializes without errors.
    #[test]
    fn test_tokio_runtime_creation() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();
        assert!(runtime.is_ok());
    }

    // VRD run function tests
    /// Tests the success case of the vrd::run() function.
    #[test]
    fn test_vrd_run_success() {
        // This test assumes that vrd::run() is exposed and can be called directly
        // You might need to modify the visibility of vrd::run() for this test to work

        // Act
        let result = vrd::run();

        // Assert
        assert!(result.is_ok());
    }

    // /// Tests error handling in the main function by simulating a panic and verifying that it is caught.
    // #[test]
    // fn test_main_error_handling() {
    //     // We'll use panic::catch_unwind to simulate and catch a panic that would occur
    //     // if vrd::run() returns an error
    //     let result = panic::catch_unwind(|| {
    //         // Create a mock vrd::run() that always returns an error
    //         let mock_run =
    //             || -> Result<(), Box<dyn std::error::Error>> {
    //                 Err("Simulated error".into())
    //             };

    //         // Call a modified version of main() that uses our mock_run
    //         let date = DateTime::new();
    //         let uuid = Uuid::new_v4().to_string();
    //         let iso = date.iso_8601;

    //         let runtime = tokio::runtime::Builder::new_current_thread()
    //             .enable_all()
    //             .build()
    //             .expect("Failed to build Tokio runtime");

    //         if let Err(run_error) = mock_run() {
    //             let error_message = format!(
    //                 "Unexpected error running vrd: {:?}",
    //                 run_error
    //             );
    //             let log_entry = create_log_entry(
    //                 &uuid,
    //                 &iso,
    //                 LogLevel::ERROR,
    //                 &error_message,
    //             );

    //             runtime
    //                 .block_on(async {
    //                     log_entry_async(log_entry).await
    //                 })
    //                 .expect("Failed to log error");

    //             // Simulating a panic instead of process::exit
    //             panic!("Simulated process::exit(1)");
    //         }
    //     });

    //     // Assert that a panic occurred (simulating process::exit(1))
    //     if let Err(err) = result {
    //         if let Some(panic_message) =
    //             err.downcast_ref::<&'static str>()
    //         {
    //             assert_eq!(
    //                 panic_message,
    //                 &"Simulated process::exit(1)"
    //             );
    //             println!(
    //                 "Test passed as panic was expected and caught"
    //             );
    //         } else if let Some(panic_message) =
    //             err.downcast_ref::<String>()
    //         {
    //             assert_eq!(panic_message, "Simulated process::exit(1)");
    //             println!(
    //                 "Test passed as panic was expected and caught"
    //             );
    //         } else {
    //             panic!(
    //             "Test failed: Panic occurred with an unexpected payload"
    //         );
    //         }
    //     } else {
    //         panic!(
    //             "Test failed: Expected a panic, but it did not occur"
    //         );
    //     }
    // }
}
