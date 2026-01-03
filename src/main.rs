use std::ffi::c_void;
use jni::objects::{JObject, JString};
use jni::{JNIEnv, JavaVM};
use std::path::Path;

fn main() {
    // Add the Java library to the class path.
    let jar_path = Path::new("java/lib/build/libs/lib.jar");
    let classpath_option = format!("-Djava.class.path={}", jar_path.display());
    let jvm_args = jni::InitArgsBuilder::new()
        .version(jni::JNIVersion::V8)
        .option(&classpath_option)
        .build()
        .expect("Failed to create JVM args");
    // Launch the JVM.
    let jvm = JavaVM::new(jvm_args).expect("Failed to create JVM");

    // The JNI interface pointer (JNIEnv) is valid only in the current thread. We need to attach it.
    // See more:
    // https://docs.oracle.com/en/java/javase/21/docs/specs/jni/invocation.html#attaching-to-the-vm
    // Detachment happens automatically when `env` is deleted.
    let mut env = jvm
        .attach_current_thread()
        .expect("Failed to attach current thread");

    // Find the native callback class and register the native method for it.
    let native_callback_class = env
        .find_class("me/ivanyu/java_library/NativeCallback")
        .expect("Failed to find NativeCallback class");
    // The signature means
    // "one argument of the class `me.ivanyu.java_library.Result`, returning `void`".
    // More details on signatures:
    // https://docs.oracle.com/en/java/javase/21/docs/specs/jni/types.html
    let native_methods = [jni::NativeMethod {
        name: jni::strings::JNIString::from("onResult"),
        sig: jni::strings::JNIString::from("(Lme/ivanyu/java_library/Result;)V"),
        fn_ptr: Java_me_ivanyu_java_1library_NativeCallback_onResult as *mut c_void,
    }];
    env.register_native_methods(&native_callback_class, &native_methods)
        .expect("Failed to register native methods");

    // Create an instance of NativeCallback
    let callback_obj = env
        // Our native callback code.
        .alloc_object(&native_callback_class)
        .expect("Failed to allocate NativeCallback object");

    // Find the Calculator class and call the `add` method (static).
    let calculator_class = env
        .find_class("me/ivanyu/java_library/Calculator")
        .expect("Failed to find Calculator class");
    env.call_static_method(
        &calculator_class,
        "add",
        "(IILme/ivanyu/java_library/ResultCallback;)V",
        &[
            jni::objects::JValue::Int(3),
            jni::objects::JValue::Int(5),
            jni::objects::JValue::Object(&callback_obj),
        ],
    )
    .expect("Failed to call add");
}

// Prevent symbol mangling
// https://doc.rust-lang.org/rustc/symbol-mangling/index.html
#[unsafe(no_mangle)]
// `extern "system"` ensures the correct calling convention for JNI.
// The name follows the convention from
// https://docs.oracle.com/en/java/javase/21/docs/specs/jni/design.html#resolving-native-method-names
// However, generally this is not needed because we use RegisterNatives function
// (through `jni` crate).
pub extern "system" fn Java_me_ivanyu_java_1library_NativeCallback_onResult(
    mut env: JNIEnv,
    _obj: JObject,
    result_obj: JObject,
) {
    // Call getMessage on the result.
    // The signature means
    // "no arguments, returns `java.lang.String`".
    // More details on signatures:
    // https://docs.oracle.com/en/java/javase/21/docs/specs/jni/types.html
    let message_jstring = env
        .call_method(result_obj, "getMessage", "()Ljava/lang/String;", &[])
        .expect("Failed to call getMessage")
        .l()  // unwrap to Object (implicitly check for null)
        .expect("getMessage returned null");
    // Convert a Java string to Rust string.
    let message_jstring = JString::from(message_jstring);
    let message: String = env
        .get_string(&message_jstring)
        .expect("Couldn't get Java string")
        .into();

    println!("{}", message);
}
