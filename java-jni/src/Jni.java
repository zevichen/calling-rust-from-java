import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * @author zv 2023/10/15 00:17
 */
public class Jni {
    static {
        System.load("/Users/zv/work/cross/calling-rust-from-java/java-jni/dylib/librust_jni.dylib");
    }

    public native String sayHello();
}
