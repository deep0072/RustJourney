# mini-grep Tool

This repository contains a Rust implementation of a grep-like tool. The tool takes a query and a file path as input, reads the file, and prints out all lines that contain the query.

## Usage

To use this tool, you need to provide the query and the file path as command-line arguments. Here is an example:

```bash
cargo run "search_term" "/path/to/file"
```

Replace `"search_term"` with the term you want to search for, and replace `"/path/to/file"` with the path to the file you want to search in.

The tool will then print out all lines in the specified file that contain the search term.

Please ensure that you have Rust installed on your system to run this tool. If you don't have Rust installed, you can download it from the [official website](https://www.rust-lang.org/tools/install).