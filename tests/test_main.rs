// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use rlg::{log_format::LogFormat, log_level::LogLevel};
    use uuid::Uuid;
    use vrd::{create_log_entry, log_entry_async};

    // Test creating a log entry
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

    // Test logging asynchronously
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

    // Test UUID generation
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
}
