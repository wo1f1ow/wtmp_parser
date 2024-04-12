# wtmp_parser

`wtmp_parser` is a Rust program that processes `wtmp` files (binary files containing user login records) and outputs the data in a human-readable format.

## Building

To build the program, you'll need to have the Rust toolchain installed. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, follow these steps:

1. Clone this repository or download the source code.
2. Navigate to the project directory.
3. Build the release version of the program:

```
cargo build --release
```

This will compile the program and place the binary in the `target/release` directory.

## Usage

```
wtmp_parser [-f wtmp_file]
```

- If no `-f` flag is provided, the program will read from `/var/log/wtmp`.
- If the `-f` flag is provided, it should be followed by the path to the desired `wtmp` file.

## Examples

### Read from the default /var/log/wtmp file
```
./target/release/wtmp_parser
```

### Read from a specific wtmp file
```
./target/release/wtmp_parser -f /path/to/your/wtmp/file
```

The program will output the records from the `wtmp` file in the following format:
```
<timestamp> <record_type> <username> <line/terminal> <hostname>
```

For example:
```
2023-04-10 10:12:34 Login      alice     tty1     example.com
2023-04-10 10:15:22 Normal     bob        pts/0    remote.host
2023-04-10 10:20:45 Term       charlie   tty2     localhost
```

## Dependencies

This program uses the following external crate:

- `chrono` (0.4.23): A crate for handling dates and times in Rust.
