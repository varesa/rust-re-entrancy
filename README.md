```
$ grep unsafe . -R
$ 
$ cargo run --example single_threaded
   Compiling rust-re-entrancy v0.1.0 (/home/esav.fi/esa/workspace/rust-re-entrancy)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/examples/single_threaded`
1
1
$ cargo run --example multi_threaded
   Compiling rust-re-entrancy v0.1.0 (/home/esav.fi/esa/workspace/rust-re-entrancy)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/examples/multi_threaded`
Result: 1
```
