# Rust + WebAssembly = ❤️

### Setup

Follow this guide: https://rustwasm.github.io/book/setup.html

### Terminal

```
$ cd rust-bin
$ cargo build [--release]
$ cargo run // or ./target/release/hello_world
```

### Web

```
$ cd web-hello-world
$ make
$ python3 server.py
```

Visit http://localhost:9000 and you are good to go

### Benchmark 💪

Time to calculate fib(40) = 102334155

Rust bin: ~200ms

#### Chrome

Rust: ~318ms

JavaScript: ~945ms

#### Firefox

Rust: ~306ms

JavaScript: ~496ms

#### Safari

Rust: ~282ms

JavaScript: ~5189ms 🙀
