// Copyright © 2023-2024 Random (VRD) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is part of the `Random (VRD)` library, a Rust implementation of the Mersenne Twister RNG.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use rlg::{log_format::LogFormat, log_level::LogLevel};
    use vrd::create_log_entry;

    #[test]
    fn test_create_log_entry() {
        let uuid = "test-uuid";
        let iso = "2023-06-10T12:34:56Z";
        let level = LogLevel::INFO;
        let message = "Test log message";

        let log_entry = create_log_entry(uuid, iso, level, message);

        assert_eq!(log_entry.session_id, uuid);
        assert_eq!(log_entry.time, iso);
        assert_eq!(log_entry.level.to_string(), "INFO");
        assert_eq!(log_entry.component, "VRD");
        assert_eq!(log_entry.description, message);
        assert_eq!(log_entry.format, LogFormat::JSON);
    }

    #[tokio::test]
    async fn test_log_entry_async() {
        let uuid = "test-uuid";
        let iso = "2023-06-10T12:34:56Z";
        let level = LogLevel::INFO;
        let message = "Test log message";
        let log_entry = create_log_entry(uuid, iso, level, message);

        // Assert that the log entry was logged successfully
        assert!(log_entry.session_id == "test-uuid");
        assert!(log_entry.time == "2023-06-10T12:34:56Z");
        assert!(log_entry.level == LogLevel::INFO);
        assert!(message == "Test log message");
    }
}
