# calling-rust-from-java

Calling rust from java

```shell
cd rust-jni && cargo build
cp target/debug/librust_jni.dylib ../java-jni/dylib/

# config
-Djava.library.path=<absolute path of dylib>
# run java main method
```
