# Symmetry Implementation for Pair Distribution function

This repository contains all code for my bachelor thesis.

The program I developed in this work can be used to calculate pair multiplicities in crystals.
For more information about the applications of this program, the mathematics behind crystals and the implementation, please refer to the [report](./report/main.pdf).

## Using the code

### Website

The simplest way to use the program is on the [website](https://max-kay.github.io/bachelor_thesis/).
Using WASM the website allows the use of the program on any machine with a browser.


### Building from Source

To build the program from source, the Rust tool chain needs to be installed.
Instructions for this can be found here: [rustup.rs](https://rustup.rs/)

After installing the Rust tool chain, the repository can be cloned using:
```
git clone https://github.com/max-kay/bachelor_thesis.git
```

The code can now be compiled using `cargo build --release` in the root directory of the repository.
This produces the executable `target/release/find-pairs`.
For more information on how to use the command line tool run `find-pairs --help`
