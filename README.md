# cgrep

A toy implementation of grep as part of reading the [The Rust Programming Language](https://doc.rust-lang.org/stable/book/second-edition/) book.


# Usage
```
cgrep pattern glob_path
```

# Example
```
 cargo run How "p*.txt"
config Config { pattern: "How", path: "p*.txt" }
poem.txt:How dreary to be somebody!
poem.txt:How public, like a frog
poem2.txt:How dreary to be somebody!
poem2.txt:How public, like a frog
```
