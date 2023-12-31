extern crate libc;

use libc::c_void;

#[allow(non_snake_case)]
#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,

    GetVersion: extern fn(env: *mut JNIEnv) -> i32,

    _opaque_data: [u8; 1824],
}

pub type JNIEnv = *const JNINativeInterface;

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn Java_Jni_sayHello(jre: *mut JNIEnv, class: *const c_void) {
    println!("Invoked native method, jre: {:p}, class: {:p}", jre, class);
    unsafe {
        let v = ((**jre).GetVersion)(jre);
        println!("version: {:?}", v);
    }
}