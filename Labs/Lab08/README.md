# Smart Pointers in Rust

## `Box` - The Heap Allocator

### Characteristics:

- Provides unique ownership of data stored on the heap.
- The pointer (the `Box` itself) lives on the stack, but the data it points to is on the heap.
- It has zero runtime overhead (other than allocation).

### Problem Solved:

- **Recursive Types**: When a type needs to include itself (e.g., a Linked List or Tree), the compiler needs to know the exact size. A `Box` has a known, fixed size (pointer size).
- **Transferring Ownership**: Moving large amounts of data without copying the data itself (only the pointer moves).
- **Trait Objects**: When you want a collection of different types that implement the same trait (`Box<dyn Trait>`).

### Example:

```rust
// Recursive type definition
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let b = Box::new(5); // 5 is stored on the heap
```

## `Rc` - Reference Counted

### Characteristics:

- Enables multiple ownership of the same data.
- Keeps track of the number of references to the data. The data is dropped only when the count reaches zero.
- Single-threaded only (not `Send` or `Sync`).

### Problem Solved:

- Scenarios where a value needs to be used by multiple parts of the program, and we don't know at compile time which part will finish using the data last (e.g., graph nodes).

## Common Usage:

- Often combined with `RefCell<T>` (`Rc<RefCell<T>>`) to allow mutation of shared data.

### Example:

```rust
use std::rc::Rc;
let original = Rc::new(5);
let copy1 = Rc::clone(&original); // ref count = 2
let copy2 = Rc::clone(&original); // ref count = 3
```

## `Weak` - The Cycle Breaker

### Characteristics:

- A non-owning reference to an allocation managed by `Rc`.
- Does not increase the strong reference count.
- Must be "upgraded" to an `Rc` (via `.upgrade()`) to access the data, which returns an `Option` (because the data might have been dropped).

### Problem Solved:

- **Memory Leaks (Reference Cycles)**: Prevents situations where two `Rc` pointers reference each other, ensuring the reference count never reaches zero.

### Example:

- Parent-child relationships in trees: Parent owns Child (`Rc`), Child points back to Parent (`Weak`).

## `Arc` - Atomic Reference Counted

### Characteristics:

- The thread-safe version of `Rc`.
- Uses atomic operations for reference counting, which adds a small performance penalty compared to `Rc`.
- Implements `Send` and `Sync`.

### Problem Solved:

- Sharing ownership of data across multiple threads.

### Common Usage:

- Often combined with `Mutex<T>` or `RwLock<T>` (`Arc<Mutex<T>>`) to share mutable state across threads.

### Example:

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let reference = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", reference);
});
```

## `Cell` - Interior Mutability via Copying

### Characteristics:

- Allows mutation of data inside an immutable struct (`&self`).
- Works by copying values in and out (`get` and `set`) or swapping them.
- Does not give references (`&T`) to the inner data.
- Single-threaded only.

### Problem Solved:

- Mutating small fields (flags, counters) within an immutable structure without the overhead of runtime borrow checking.

### Example:

```rust
use std::cell::Cell;
struct Component {
    active: Cell<bool>,
}
// can allow modification even if 'Component' is borrowed immutably
```

## `RefCell` - Interior Mutability via Borrowing

### Characteristics:

- Allows mutation of data inside an immutable struct.
- Enforces Rust's borrowing rules (one mutable OR many immutable) at runtime instead of compile time.
- Panics if rules are violated (e.g., two `borrow_mut()` active at once).
- Single-threaded only.

### Problem Solved:

- Mutating complex data (like `Vec` or `String`) shared via `Rc`.

### Example:

```rust
use std::cell::RefCell;
let data = RefCell::new(5);
*data.borrow_mut() += 1;
```

## `Cow` - Clone-On-Write

### Characteristics:

- A smart enum that can hold either a `Borrowed` reference (`&T`) or an `Owned` value (`T`).
- Only clones (allocates) the data when you try to modify it (write).

### Problem Solved:

- **Optimization**: Avoids unnecessary memory allocation when data is mostly read and rarely modified.

### Example:

- `String` processing functions (e.g., escaping symbols) where the output is usually identical to the input.

```rust
use std::borrow::Cow;
fn sanitize(input: &str) -> Cow<str> {
    if input.contains("bad") {
        Cow::Owned(input.replace("bad", "***"))
    } else {
        Cow::Borrowed(input)
    }
}
```

## `OnceCell` - Initialize Once

### Characteristics:

- A cell that can be written to exactly one time.
- Subsequent attempts to write will fail.
- Subsequent reads will return the same value.
- Single-threaded only.

### Problem Solved:

- Lazy initialization of fields in structs where the value isn't known at construction time.
- Initializing data that requires calculation without passing `&mut`.

### Example:

```rust
use std::cell::OnceCell;
let cell = OnceCell::new();
let val = cell.get_or_init(|| 99);
```

## `OnceLock` - Thread-Safe Initialize Once

### Characteristics:

- Blocks other threads if initialization is currently happening.

### Problem Solved:

- **Global State**: Creating global static singletons safely.

### Example:

```rust
use std::sync::OnceLock;
static CONFIG: OnceLock<Config> = OnceLock::new();
fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| load_config())
}
```

## `LazyCell` - Automated Lazy Initialization

### Characteristics:

- Holds both the data (optionally) and the initialization function (closure).
- Automatically runs the initializer upon the first access (dereference).
- Single-threaded only.

### Problem Solved:

- **Ergonomics**: Unlike `OnceCell`, you define how to initialize it when creating the variable, not when accessing it.

### Example:

```rust
use std::cell::LazyCell;
let lazy = LazyCell::new(|| complex_calculation());
println!("{}", *lazy); // Calculation happens here
```

## `LazyLock` - Thread-Safe Automated Lazy Initialization

### Characteristics:

- The thread-safe (`Sync`) version of `LazyCell`.

### Problem Solved:

- Defining lazy global static variables with minimal boilerplate.

### Example:

```rust
use std::sync::LazyLock;
static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(1, "foo".to_string());
    m
});
```
