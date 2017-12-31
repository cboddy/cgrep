# cgrep

A toy implementation of grep as part of reading the [The Rust Programming Language](https://doc.rust-lang.org/stable/book/second-edition/) book.


# Usage
```
cgrep pattern_re glob_path
```

# Example
```
cargo run "^H"  "p*.txt"
```
or 
```
cargo run How "p*.txt"
```
will produce output formatted thusly
```
config Config { pattern: "How", path: "p*.txt" }
poem.txt:How dreary to be somebody!
poem.txt:How public, like a frog
poem2.txt:How dreary to be somebody!
poem2.txt:How public, like a frog
```
