# dorustos

A simple CHIP-8 emulator written in Rust, following the steps outlined at [An Introduction to Chip-8 Emulation using the Rust Programming Language](https://github.com/aquova/chip8-book).

Built during a weekend afternoon with a big bag of Doritos (do Doritos count as chips?) while learning Rust from scratch :-)

## Installation

In order to install this emulator you'll need `libsdl2` installed to your system. You should be able to achieve this by using `brew`, `apt` or your chosen OS's package manager.

Clone the repo and use `cargo` to build this project.

```bash
cargo build
```

Alternatively, you can install the project to Cargo's binaries folder.

```bash
cargo install --path .
```

## Usage

In order to use the emulator you'll need to find (or create!) a Chip-8 ROM. You can find these online, and the repo / guide linked above has [a few ROMs](https://github.com/aquova/chip8-book/tree/master/roms) you can use to try it out.

After building, run the project by specifying the path to the file containing your ROM:

```bash
cargo run <FILE>

# Example
cargo run roms/CONNECT4
```

If you installed the project instead, you can directly use the `dorustos` executable.

```bash
dorustos <FILE>

# Example
dorustos roms/CONNECT4
```

## Contributing

Pull requests, bug reports and discussions are welcome (and encouraged!). Please use this repo's issues to start any discussions and I'll try to respond as soon as possible.

While there's no required coverage enforcement, I'd like to slowly add tests to (most of) the repo's codebase.

## License

This codebase is licensed under the [GNU Affero General Public License](./LICENSE).
