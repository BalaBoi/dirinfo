# Dirinfo

`dirinfo` is a simple command-line tool written in Rust that provides information about the number of files in a specified directory. If no directory path is provided, it defaults to analyzing the current directory.

## Features

- Accepts a directory path as input and returns the **number of files** within it.
- Defaults to the **current directory** if no path is provided.
- Efficiently handles directory traversal.

## Installation

To install the tool, you need to have [Rust](https://www.rust-lang.org/) installed on your system. Once Rust is installed, you can build the tool using `cargo`:

```bash
# Clone the repository
$ git clone https://github.com/BalaBoi/dirinfo

# Navigate into the project directory
$ cd dirinfo

# Install the tool
$ cargo install --path .

# Check the tool is installed
$ dirinfo --help
```

## Usage

Run the `dirinfo` command with or without a directory path:

### Examples

1. **Analyze the current directory**:

   ```bash
   $ dirinfo
   Number of files: 42
   ```

2. **Analyze a specific directory**:

   ```bash
   $ dirinfo /path/to/directory
   Number of files: 128
   ```

## Future Enhancements

- Calculate the **total size** of files in the directory.
- Provide detailed statistics (e.g., file types, subdirectory counts).
- Add recursive or non-recursive options.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
