# Website Checker

A concurrent website monitoring tool built in Rust.

## Build Instructions

To build the project in release mode, run:

```sh
cargo build --release
```

The compiled binary will be located at `target/release/website_checker`.

## Usage

1. Create a `sites.txt` file in the project root directory, listing one website URL per line. Example:

    ```
    https://www.rust-lang.org
    https://www.example.com
    ```

2. Run the program:

    ```sh
    cargo run --release
    ```

    Or, after building:

    ```sh
    ./target/release/website_checker
    ```

The program will read each URL from `sites.txt` and print the HTTP status for each website.

## Bonus Features

- **Concurrent Requests:** The tool is designed to be easily extended for concurrent website checks, improving speed for large lists.
- **Customizable Input:** You can modify `sites.txt` to check any set of websites.
- **Error Reporting:** If a website is unreachable or returns an error, the tool will display the error message alongside the URL.

Feel free to extend the tool for more advanced monitoring or reporting features!
