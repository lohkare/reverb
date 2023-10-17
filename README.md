# Reverb - `echo` but in Rust.

A simple implementation for printing text in terminal done with Rust.

Made just to learn more Rust. Nothing fancy.

```
Print input to stdout

Usage: reverb [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Input to be printed

Options:
  -c, --copy      Copy the stdout print to your clipboard. Cannot be used with separate.
                  Copy will take priority over separate.
  -s, --separate  Whether to print the input on one line or multiple.
                  Does not take effect if used with copy option.
  -h, --help      Print help

```

### Installation

Simply clone the repository and run the command `cargo install --path .` in the project root.
You might need to add cargo installation path to `PATH`.