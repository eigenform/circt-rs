# Rust bindings for CIRCT

This project contains Rust bindings for [CIRCT](https://github.com/llvm/circt).

## Dependencies

This project relies on relatively recent versions of the following. Versions tested are in parentheses.

* Git   (2.39.0)
* Cargo (1.73.0)
* CMake (3.26.4)
* Ninja (1.10.2)

## Building

To build this project, initialize or update the CIRCT submodule if necessary:

```
git submodule update --init --recursive
```

Then, use the `utils/build.sh` script to compile CIRCT and run `cargo build`:

```
. utils/build.sh
```
