# Stack Direction

Checks which direction the stack grows. It calls a first function, gets a reference to a stack variable and then calls a second function from within the first function and compares the value of the reference with a stack variable in the second function. The references are both cast to usize. If the variables are decreasing in size (expected on Intel/linux boxes) then the pointer will get smaller.

Note the functions needs to be set with 

```#[inline(never)]```

else the optimizer messes with layout

Example results on an Intel linux

```
vscode ➜ /workspaces/neb-rust-devbook/ch4_stack (main) $ cargo run --bin  ch4_stack
   Compiling ch4_stack v0.1.0 (/workspaces/neb-rust-devbook/ch4_stack)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/workspaces/neb-rust-devbook/target/debug/ch4_stack`
Stack pointer decreases 140736815541804 > 140736815541340

vscode ➜ /workspaces/neb-rust-devbook/ch4_stack (main) $ cargo run --bin  ch4_stack --release
   Compiling ch4_stack v0.1.0 (/workspaces/neb-rust-devbook/ch4_stack)
    Finished release [optimized] target(s) in 0.17s
     Running `/workspaces/neb-rust-devbook/target/release/ch4_stack`
Stack pointer decreases 140723593006884 > 140723593006788
```

