# Red-Black Tree Implementation

This project implements a **Red-Black Tree** dictionary in **Rust**, designed with strict memory constraints and interoperability in mind. It operates without the standard library's allocation tools (no `Box`, `Vec`, `String`, or `BTreeMap`), relying instead on manual memory management via `libc` (`malloc`/`free`).

Complete Red-Black Tree insertion and deletion algorithms logic is based on [CLRS (Introduction to Algorithms)](https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf), ensuring $O(\log n)$ performance.

The project exposes a C-compatible API.

## Project Components

This repository consists of three main parts:

1.  **The Core Library (`src/lib.rs`)**
    The actual implementation of the Red-Black Tree and the C-API exports.
2.  **Rust Usage Example**
    A demonstration of how to use the library natively within Rust.
    - Source: [`./src/main.rs`](./src/main.rs)
3.  **C Usage Example (Interoperability)**
    A C program that links against the Rust library, demonstrating cross-language compatibility.
    - Source: [`./main.c`](./main.c)

## Prerequisites

Before running the examples, ensure you have the following installed:

- **Rust Toolchain:** Latest stable version (via `rustup` / `cargo`).
- **C Compiler:** GCC (e.g., MinGW-w64 for Windows).

## Building and Running

To build and run the Rust demonstration in `release` mode:

```
cargo run --release
```

To run the C example, you need to compile the Rust library as a dynamic library (DLL) and then link it with the C code.

**Step 1:** Build the Rust Library

This generates the `.dll` (and `.lib`) files in the target directory:

```
cargo build --release
```

**Step 2:** Prepare the Artifacts

Copy the compiled library to the project root so the C compiler and the executable can find it:

```
copy target\release\red_black_tree.dll .
```

**Step 3:** Compile the C Executable

Use GCC to compile `main.c` and link it against the Rust library:

```
gcc main.c -L. -lred_black_tree -o c_red_black_tree.exe
```

**Step 4:** Run

Execute the resulting binary:

```
.\c_red_black_tree.exe
```

## License

This project is licensed under the MIT License.

## Author

The project was created by [Adam Grącikowski](https://github.com/adamgracikowski).
