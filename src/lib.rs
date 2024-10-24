use jni::JNIEnv;
use payload::Payload;
use jni::objects::{JClass, JString};

use jni::sys::jstring;

mod payload;
mod chromeos_update_engine;

#[no_mangle]
pub extern "system" fn Java_com_rajmani7584_payloaddumper_MainActivity_getPartitionList<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, path: JString<'local>) -> jstring {

    let mut payload = Payload::new(env.get_string(&path).expect("err msg").into());
    let result = payload.init();
    let res = result.unwrap().to_string();
    let msg = env.new_string(res).expect("error msg 2").into_raw();

    return msg;
}