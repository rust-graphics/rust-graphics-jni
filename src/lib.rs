use std::os::raw::{c_char, c_int, c_longlong, c_short, c_uchar, c_ushort, c_void};

pub type JBoolean = c_uchar;
pub type JByte = c_char;
pub type JChar = c_ushort;
pub type JShort = c_short;
pub type JInt = c_int;
pub type JLong = c_longlong;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JSize = JInt;
pub type JObject = *mut c_void;
pub type JClass = JObject;
pub type JString = JObject;
pub type JArray = JObject;
pub type JObjectArray = JArray;
pub type JBooleanArray = JArray;
pub type JByteArray = JArray;
pub type JCharArray = JArray;
pub type JShortArray = JArray;
pub type JIntArray = JArray;
pub type JLongArray = JArray;
pub type JFloatArray = JArray;
pub type JDoubleArray = JArray;
pub type JThrowable = JObject;
pub type JWeak = JObject;
pub type JFieldID = *mut c_void;
pub type JMethodID = *mut c_void;
pub type JValue = u64;

#[repr(C)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fn_ptr: *mut c_void,
}

#[repr(u32)]
pub enum JObjectRefType {
    JNIInvalidRefType = 0,
    JNILocalRefType = 1,
    JNIGlobalRefType = 2,
    JNIWeakGlobalRefType = 3,
}

#[repr(C)]
pub struct JNIInvokeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub destroy_java_vm: &'static mut extern "C" fn(arg1: &mut JavaVM) -> JInt,
    pub attach_current_thread: &'static mut extern "C" fn(
        arg1: &mut JavaVM,
        arg2: &'static mut &'static mut JNIEnv,
        arg3: *mut c_void,
    ) -> JInt,
    pub detach_current_thread: &'static mut unsafe extern "C" fn(arg1: &mut JavaVM) -> JInt,
    pub get_env: &'static mut unsafe extern "C" fn(
        arg1: &mut JavaVM,
        arg2: *mut *mut c_void,
        arg3: JInt,
    ) -> JInt,
    pub attach_current_thread_as_daemon: &'static mut extern "C" fn(
        arg1: &mut JavaVM,
        arg2: &'static mut &'static mut JNIEnv,
        arg3: *mut c_void,
    ) -> JInt,
}

#[repr(C)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,
    pub get_version: &'static mut extern "C" fn(arg1: &mut JNIEnv) -> JInt,
    pub define_class: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: *const c_char,
        arg3: JObject,
        arg4: *const JByte,
        arg5: JSize,
    ) -> JClass,
    pub find_class: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *const c_char) -> JClass,
    pub from_reflected_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JMethodID,
    pub from_reflected_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JFieldID,
    pub to_reflected_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: JBoolean,
    ) -> JObject,
    pub get_superclass: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass) -> JClass,
    pub is_assignable_from:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JClass) -> JBoolean,
    pub to_reflected_field: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JFieldID,
        arg4: JBoolean,
    ) -> JObject,
    pub throw: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JThrowable) -> JInt,
    pub throw_new:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: *const c_char) -> JInt,
    pub exception_occurred: &'static mut extern "C" fn(arg1: &mut JNIEnv) -> JThrowable,
    pub exception_describe: &'static mut extern "C" fn(arg1: &mut JNIEnv),
    pub exception_clear: &'static mut extern "C" fn(arg1: &mut JNIEnv),
    pub fatal_error: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *const c_char),
    pub push_local_frame: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JInt) -> JInt,
    pub pop_local_frame: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JObject,
    pub new_global_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JObject,
    pub delete_global_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject),
    pub delete_local_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject),
    pub is_same_object:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JObject) -> JBoolean,
    pub new_local_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JObject,
    pub ensure_local_capacity: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JInt) -> JInt,
    pub alloc_object: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass) -> JObject,
    pub new_object: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        ...
    ) -> JObject,
    pub new_object_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JObject,
    pub new_object_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JObject,
    pub get_object_class: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JClass,
    pub is_instance_of:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JClass) -> JBoolean,
    pub get_method_id: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: *const c_char,
        arg4: *const c_char,
    ) -> JMethodID,
    pub call_object_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        ...
    ) -> JObject,
    pub call_object_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JObject,
    pub call_object_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JObject,
    pub call_boolean_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        ...
    ) -> JBoolean,
    pub call_boolean_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JBoolean,
    pub call_boolean_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JBoolean,
    pub call_byte_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JMethodID, ...) -> JByte,
    pub call_byte_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JByte,
    pub call_byte_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JByte,
    pub call_char_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JMethodID, ...) -> JChar,
    pub call_char_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JChar,
    pub call_char_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JChar,
    pub call_short_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        ...
    ) -> JShort,
    pub call_short_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JShort,
    pub call_short_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JShort,
    pub call_int_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JMethodID, ...) -> JInt,
    pub call_int_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JInt,
    pub call_int_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JInt,
    pub call_long_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JMethodID, ...) -> JLong,
    pub call_long_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JLong,
    pub call_long_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JLong,
    pub call_float_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        ...
    ) -> JFloat,
    pub call_float_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JFloat,
    pub call_float_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JFloat,
    pub call_double_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        ...
    ) -> JDouble,
    pub call_double_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JDouble,
    pub call_double_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JDouble,
    pub call_void_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JMethodID, ...),
    pub call_void_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut c_void,
    ),
    pub call_void_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JMethodID,
        arg4: *mut JValue,
    ),
    pub call_nonvirtual_object_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JObject,
    pub call_nonvirtual_object_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JObject,
    pub call_nonvirtual_object_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JObject,
    pub call_nonvirtual_boolean_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JBoolean,
    pub call_nonvirtual_boolean_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JBoolean,
    pub call_nonvirtual_boolean_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JBoolean,
    pub call_nonvirtual_byte_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JByte,
    pub call_nonvirtual_byte_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JByte,
    pub call_nonvirtual_byte_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JByte,
    pub call_nonvirtual_char_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JChar,
    pub call_nonvirtual_char_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JChar,
    pub call_nonvirtual_char_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JChar,
    pub call_nonvirtual_short_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JShort,
    pub call_nonvirtual_short_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JShort,
    pub call_nonvirtual_short_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JShort,
    pub call_nonvirtual_int_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JInt,
    pub call_nonvirtual_int_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JInt,
    pub call_nonvirtual_int_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JInt,
    pub call_nonvirtual_long_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JLong,
    pub call_nonvirtual_long_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JLong,
    pub call_nonvirtual_long_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JLong,
    pub call_nonvirtual_float_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JFloat,
    pub call_nonvirtual_float_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JFloat,
    pub call_nonvirtual_float_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JFloat,
    pub call_nonvirtual_double_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ) -> JDouble,
    pub call_nonvirtual_double_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ) -> JDouble,
    pub call_nonvirtual_double_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ) -> JDouble,
    pub call_nonvirtual_void_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        ...
    ),
    pub call_nonvirtual_void_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut c_void,
    ),
    pub call_nonvirtual_void_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JClass,
        arg4: JMethodID,
        arg5: *mut JValue,
    ),
    pub get_field_id: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: *const c_char,
        arg4: *const c_char,
    ) -> JFieldID,
    pub get_object_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JObject,
    pub get_boolean_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JBoolean,
    pub get_byte_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JByte,
    pub get_char_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JChar,
    pub get_short_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JShort,
    pub get_int_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JInt,
    pub get_long_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JLong,
    pub get_float_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JFloat,
    pub get_double_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID) -> JDouble,
    pub set_object_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JObject),
    pub set_boolean_field: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObject,
        arg3: JFieldID,
        arg4: JBoolean,
    ),
    pub set_byte_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JByte),
    pub set_char_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JChar),
    pub set_short_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JShort),
    pub set_int_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JInt),
    pub set_long_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JLong),
    pub set_float_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JFloat),
    pub set_double_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject, arg3: JFieldID, arg4: JDouble),
    pub get_static_method_id: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: *const c_char,
        arg4: *const c_char,
    ) -> JMethodID,
    pub call_static_object_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        ...
    ) -> JObject,
    pub call_static_object_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JObject,
    pub call_static_object_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JObject,
    pub call_static_boolean_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        ...
    ) -> JBoolean,
    pub call_static_boolean_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JBoolean,
    pub call_static_boolean_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JBoolean,
    pub call_static_byte_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JByte,
    pub call_static_byte_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JByte,
    pub call_static_byte_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JByte,
    pub call_static_char_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JChar,
    pub call_static_char_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JChar,
    pub call_static_char_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JChar,
    pub call_static_short_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JShort,
    pub call_static_short_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JShort,
    pub call_static_short_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JShort,
    pub call_static_int_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JInt,
    pub call_static_int_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JInt,
    pub call_static_int_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JInt,
    pub call_static_long_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JLong,
    pub call_static_long_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JLong,
    pub call_static_long_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JLong,
    pub call_static_float_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...) -> JFloat,
    pub call_static_float_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JFloat,
    pub call_static_float_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JFloat,
    pub call_static_double_method: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        ...
    ) -> JDouble,
    pub call_static_double_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ) -> JDouble,
    pub call_static_double_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ) -> JDouble,
    pub call_static_void_method:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JMethodID, ...),
    pub call_static_void_method_v: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut c_void,
    ),
    pub call_static_void_method_a: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: JMethodID,
        arg4: *mut JValue,
    ),
    pub get_static_field_id: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: *const c_char,
        arg4: *const c_char,
    ) -> JFieldID,
    pub get_static_object_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JObject,
    pub get_static_boolean_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JBoolean,
    pub get_static_byte_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JByte,
    pub get_static_char_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JChar,
    pub get_static_short_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JShort,
    pub get_static_int_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JInt,
    pub get_static_long_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JLong,
    pub get_static_float_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JFloat,
    pub get_static_double_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID) -> JDouble,
    pub set_static_object_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JObject),
    pub set_static_boolean_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JBoolean),
    pub set_static_byte_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JByte),
    pub set_static_char_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JChar),
    pub set_static_short_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JShort),
    pub set_static_int_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JInt),
    pub set_static_long_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JLong),
    pub set_static_float_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JFloat),
    pub set_static_double_field:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass, arg3: JFieldID, arg4: JDouble),
    pub new_string:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *const JChar, arg3: JSize) -> JString,
    pub get_string_length: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JString) -> JSize,
    pub get_string_chars: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JString,
        arg3: *mut JBoolean,
    ) -> *const JChar,
    pub release_string_chars:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JString, arg3: *const JChar),
    pub new_string_utf:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *const c_char) -> JString,
    pub get_string_utf_length:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JString) -> JSize,
    pub get_string_utf_chars: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JString,
        arg3: *mut JBoolean,
    ) -> *const c_char,
    pub release_string_utf_chars:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JString, arg3: *const c_char),
    pub get_array_length: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JArray) -> JSize,
    pub new_object_array: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JSize,
        arg3: JClass,
        arg4: JObject,
    ) -> JObjectArray,
    pub get_object_array_element:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObjectArray, arg3: JSize) -> JObject,
    pub set_object_array_element: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JObjectArray,
        arg3: JSize,
        arg4: JObject,
    ),
    pub new_boolean_array:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JBooleanArray,
    pub new_byte_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JByteArray,
    pub new_char_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JCharArray,
    pub new_short_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JShortArray,
    pub new_int_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JIntArray,
    pub new_long_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JLongArray,
    pub new_float_array: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JFloatArray,
    pub new_double_array:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JSize) -> JDoubleArray,
    pub get_boolean_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JBooleanArray,
        arg3: *mut JBoolean,
    ) -> *mut JBoolean,
    pub get_byte_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JByteArray,
        arg3: *mut JBoolean,
    ) -> *mut JByte,
    pub get_char_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JCharArray,
        arg3: *mut JBoolean,
    ) -> *mut JChar,
    pub get_short_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JShortArray,
        arg3: *mut JBoolean,
    ) -> *mut JShort,
    pub get_int_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JIntArray,
        arg3: *mut JBoolean,
    ) -> *mut JInt,
    pub get_long_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JLongArray,
        arg3: *mut JBoolean,
    ) -> *mut JLong,
    pub get_float_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JFloatArray,
        arg3: *mut JBoolean,
    ) -> *mut JFloat,
    pub get_double_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JDoubleArray,
        arg3: *mut JBoolean,
    ) -> *mut JDouble,
    pub release_boolean_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JBooleanArray,
        arg3: *mut JBoolean,
        arg4: JInt,
    ),
    pub release_byte_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JByteArray,
        arg3: *mut JByte,
        arg4: JInt,
    ),
    pub release_char_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JCharArray,
        arg3: *mut JChar,
        arg4: JInt,
    ),
    pub release_short_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JShortArray,
        arg3: *mut JShort,
        arg4: JInt,
    ),
    pub release_int_array_elements:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JIntArray, arg3: *mut JInt, arg4: JInt),
    pub release_long_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JLongArray,
        arg3: *mut JLong,
        arg4: JInt,
    ),
    pub release_float_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JFloatArray,
        arg3: *mut JFloat,
        arg4: JInt,
    ),
    pub release_double_array_elements: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JDoubleArray,
        arg3: &mut JDouble,
        arg4: JInt,
    ),
    pub get_boolean_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JBooleanArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JBoolean,
    ),
    pub get_byte_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JByteArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JByte,
    ),
    pub get_char_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JCharArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JChar,
    ),
    pub get_short_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JShortArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JShort,
    ),
    pub get_int_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JIntArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JInt,
    ),
    pub get_long_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JLongArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JLong,
    ),
    pub get_float_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JFloatArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JFloat,
    ),
    pub get_double_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JDoubleArray,
        arg3: JSize,
        arg4: JSize,
        arg5: &mut JDouble,
    ),
    pub set_boolean_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JBooleanArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JBoolean,
    ),
    pub set_byte_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JByteArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JByte,
    ),
    pub set_char_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JCharArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JChar,
    ),
    pub set_short_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JShortArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JShort,
    ),
    pub set_int_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JIntArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JInt,
    ),
    pub set_long_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JLongArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JLong,
    ),
    pub set_float_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JFloatArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JFloat,
    ),
    pub set_double_array_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JDoubleArray,
        arg3: JSize,
        arg4: JSize,
        arg5: *const JDouble,
    ),
    pub register_natives: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JClass,
        arg3: *const JNINativeMethod,
        arg4: JInt,
    ) -> JInt,
    pub unregister_natives: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JClass) -> JInt,
    pub monitor_enter: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JInt,
    pub monitor_exit: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JInt,
    pub get_java_vm: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *mut *mut JavaVM) -> JInt,
    pub get_string_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JString,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut JChar,
    ),
    pub get_string_utf_region: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JString,
        arg3: JSize,
        arg4: JSize,
        arg5: *mut c_char,
    ),
    pub get_primitive_array_critical: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JArray,
        arg3: *mut JBoolean,
    ) -> *mut c_void,
    pub release_primitive_array_critical:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JArray, arg3: *mut c_void, arg4: JInt),
    pub get_string_critical: &'static mut extern "C" fn(
        arg1: &mut JNIEnv,
        arg2: JString,
        arg3: *mut JBoolean,
    ) -> *const JChar,
    pub release_string_critical:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JString, arg3: *const JChar),
    pub new_weak_global_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JWeak,
    pub delete_weak_global_ref: &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JWeak),
    pub exception_check: &'static mut extern "C" fn(arg1: &mut JNIEnv) -> JBoolean,
    pub new_direct_byte_buffer:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: *mut c_void, arg3: JLong) -> JObject,
    pub get_direct_buffer_address:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> *mut c_void,
    pub get_direct_buffer_capacity:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JLong,
    pub get_object_ref_type:
        &'static mut extern "C" fn(arg1: &mut JNIEnv, arg2: JObject) -> JObjectRefType,
}

pub type JNIEnv = &'static JNINativeInterface;
pub type JavaVM = &'static JNIInvokeInterface;
