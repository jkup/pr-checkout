use std::io::{self, Write};
use std::process::{Command, exit};

fn main() {
    // Get the repository URL and PR number from the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <repository_url> <pr_number>", args[0]);
        exit(1);
    }
    let repository_url = &args[1];
    let pr_number = &args[2];

    // Clone the repository
    let status = Command::new("git")
        .args(&["clone", repository_url])
        .status();
    if let Err(e) = status {
        eprintln!("Failed to clone repository: {}", e);
        exit(1);
    }

    // Move into the repository directory
    let repository_name = repository_url.split('/').last().unwrap().trim_end_matches(".git");
    let repository_path = format!("./{}", repository_name);
    let status = Command::new("sh")
        .args(&["-c", &format!("cd {}", repository_path)])
        .status();
    if let Err(e) = status {
        eprintln!("Failed to move into repository directory: {}", e);
        exit(1);
    }

    // Fetch the pull request
    let pr_branch_name = format!("pr-{}", pr_number);
    let status = Command::new("git")
        .args(&["fetch", "origin", &format!("pull/{}/head:{}", pr_number, pr_branch_name)])
        .status();
    if let Err(e) = status {
        eprintln!("Failed to fetch pull request: {}", e);
        exit(1);
    }

    // Checkout the new branch
    let status = Command::new("git")
        .args(&["checkout", &pr_branch_name])
        .status();
    if let Err(e) = status {
        eprintln!("Failed to checkout pull request branch: {}", e);
        exit(1);
    }

    println!("Pull request fetched and checked out successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_and_checkout() {
        // Test with a valid repository URL and PR number
        let output = run_command("cargo", &["run", "--", "https://github.com/owner/repo.git", "123"]);
        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout), "Pull request fetched and checked out successfully!\n");

        // Test with an invalid repository URL
        let output = run_command("cargo", &["run", "--", "invalid_url", "123"]);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to clone repository"));

        // Test with an invalid PR number
        let output = run_command("cargo", &["run", "--", "https://github.com/owner/repo.git", "invalid_number"]);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Failed to fetch pull request"));
    }

    fn run_command(program: &str, args: &[&str]) -> std::process::Output {
        let mut command = std::process::Command::new(program);
        command.args(args);

        let output = command.output().unwrap();
        if !output.status.success() {
            panic!("Command failed: {:?}\n\n{}", command, String::from_utf8_lossy(&output.stderr));
        }

        output
    }
}
