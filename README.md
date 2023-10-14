# calling-rust-from-java

Calling rust from java

```shell
cd rust-jni && cargo build
cp target/debug/librust_jni.dylib ../java-jni/dylib/

# config
-Djava.library.path=<absolute path of dylib>
# run java main method
```

## Reference

-   [How to compile a crate as dynamic library... : rust --- （7） 如何将一个板条箱编译为动态库... ： rust (reddit.com)](https://www.reddit.com/r/rust/comments/64hlfa/how_to_compile_a_crate_as_dynamic_library/)
-   [Rust Plugins - Rust Tutorials (zicklag.github.io)](https://zicklag.github.io/rust-tutorials/rust-plugins.html)
-   [Cargo Targets - The Cargo Book (rust-lang.org)](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
-   [3180-cargo-cli-crate-type - The Rust RFC Book (rust-lang.github.io)](https://rust-lang.github.io/rfcs/3180-cargo-cli-crate-type.html)
-   [Linkage - The Rust Reference (rust-lang.org)](https://doc.rust-lang.org/reference/linkage.html)


