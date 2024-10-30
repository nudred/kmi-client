Certainly! Here’s a polished README in `.md` format in English for your project:

---

# KMI - PasteBin Alternative by Aeza

**KMI** is a Rust-based command-line utility that allows you to send text data to a server in a PasteBin-like style, retrieve your IP address, and check server connectivity. This project is designed to work with the [Aeza](https://kmi.aeza.net) service.

## Features

- **Text Data Submission**: Send text information to the server, with support for reading data from standard input.
- **Retrieve IP Address**: Get your current IP address via the server.
- **Ping the Server**: Check the connection to the server by sending a `ping` request.

## Installation

### Requirements

- Rust version 1.56 or higher

1. Clone the repository:
   ```bash
   git clone https://github.com/nudred/kmi.git
   cd kmi
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the utility from the `target/release` directory:
   ```bash
   ./target/release/kmi --help
   ```

## Usage

### Basic Commands

1. **Send text data to the server**
   ```bash
   kmi "Hello KMI!"
   ```
   Or pipe data from standard input:
   ```bash
   echo "Hello KMI" | kmi
   ```
   ```bash
   cat file.txt | kmi
   ```
2. **Retrieve your IP address**
   ```bash
   kmi --ip
   ```

3. **Ping the server**
   ```bash
   kmi --ping
   ```

### Options

- `--ip` or `-i`: Request your IP address.
- `--ping` or `-p`: Send a ping to check the connection with the server.

## Dependencies

- [clap](https://docs.rs/clap/) — Command-line argument parser
- [reqwest](https://docs.rs/reqwest/) — HTTP client
- [colored](https://docs.rs/colored/) — Text color formatting
- [atty](https://docs.rs/atty/) — TTY detection

## Contributing

Contributions are welcome! Please create an issue or submit a pull request if you have suggestions or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

--- 

This README is now in `.md` format and ready for a GitHub audience!
