# Red-Black Tree Implementation

This project implements a **Red-Black Tree** dictionary in **Rust** designed with strict memory constraints. It operates without the standard library's allocation tools (no `Box`, `Vec`, `String`, or `BTreeMap`), relying instead on manual memory management via `libc` (`malloc`/`free`).

Complete Red-Black Tree insertion and deletion algorithms (based on [CLRS](https://www.cs.mcgill.ca/~akroit/math/compsci/Cormen%20Introduction%20to%20Algorithms.pdf)) ensuring $O(\log n)$ performance.

## Prerequisites

- **Rust:** Latest stable version (`cargo`).
- **C Compiler:** GCC (MinGW for Windows).

## 1. Rust Demo

To run the rust demonstration (located in `src/main.rs`):

```
cargo run --release
```

## 2. C Demo

### Windows

Build the library (`.dll`):

```
cargo build --release
```

Copy the `.dll` to the project root:

```
copy target\release\red_black_tree.dll .
```

Compile the C executable (link the C code with the Rust library):

```
gcc main.c -L. -lred_black_tree -o c_red_black_tree.exe
```

Run the executable:

```
.\c_red_black_tree.exe
```
