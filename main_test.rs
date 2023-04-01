#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_main() {
        // Test with a valid repository URL and PR number
        let output = Command::new("cargo")
            .args(&["run", "--", "https://github.com/owner/repo.git", "123"])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Pull request fetched and checked out successfully!\n");

        // Test with an invalid repository URL
        let output = Command::new("cargo")
            .args(&["run", "--", "invalid_url", "123"])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to clone repository"));

        // Test with an invalid PR number
        let output = Command::new("cargo")
            .args(&["run", "--", "https://github.com/owner/repo.git", "invalid_number"])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to fetch pull request"));
    }
}
