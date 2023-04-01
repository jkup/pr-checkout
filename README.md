# GitHub PR Fetcher

This is a Rust command-line application that fetches and checks out a GitHub pull request based on the PR number.

## Installation

To install the application, you will need to have Rust and Cargo installed on your system. If you don't have Rust and Cargo installed, you can download them from the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust and Cargo are installed, clone the repository and navigate into the project directory:

```sh
git clone https://github.com/username/repo.git
cd repo
```

## Usage

To fetch and check out a pull request, run the following command:

```sh
cargo run -- <repository_url> <pr_number>
```

Replace <repository_url> with the URL of the GitHub repository, and <pr_number> with the number of the pull request you want to fetch and check out.

For example:

```sh
cargo run -- https://github.com/username/repo.git 123
```

This will fetch and check out pull request #123 in the username/repo repository.

## Tests

To run the tests, execute the following command:

```sh
cargo test
```

This will compile and run the tests, and display the results in the terminal.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
