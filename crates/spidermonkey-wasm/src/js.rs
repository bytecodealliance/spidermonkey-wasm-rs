use anyhow::{bail, Result};
use spidermonkey_wasm_sys::jsffi::{
    context_options_ref as raw_context_options_ref, default_global_class_ops,
    js_class_global_flags, ContextOptions, JSContext, JSObject, JSString, JSStringToRustString,
    JS_NewGlobalObject, OnNewGlobalHookOption, RealmOptions, ReportException, RunJobs, ToString,
    Utf8IsCompilableUnit,
};

use spidermonkey_wasm_sys::jsclass::JSClass;

// Re-exports of safe bindings from the sys crate
pub use spidermonkey_wasm_sys::jsffi::{make_default_realm_options, undefined_value};

use crate::handle::{HandleObject, HandleString, HandleValue};

use std::{pin::Pin, ptr};

pub fn new_global_object(
    cx: *mut JSContext,
    class: &JSClass,
    opts: &RealmOptions,
) -> *mut JSObject {
    unsafe {
        JS_NewGlobalObject(
            cx,
            class,
            ptr::null_mut(),
            OnNewGlobalHookOption::FireOnNewGlobalHook,
            opts,
        )
    }
}

pub fn make_default_global_class() -> JSClass {
    JSClass {
        name: b"global\0".as_ptr() as *const i8,
        flags: js_class_global_flags(),
        c_ops: default_global_class_ops(),
        spec: std::ptr::null(),
        ext: std::ptr::null(),
        o_ops: std::ptr::null(),
    }
}

pub fn run_jobs(cx: *mut JSContext) {
    unsafe {
        RunJobs(cx);
    }
}

pub fn report_exception(cx: *mut JSContext) -> Result<()> {
    if !unsafe { ReportException(cx) } {
        bail!("Exception thrown while reporting exception");
    }

    Ok(())
}

pub fn to_string(cx: *mut JSContext, val: HandleValue) -> *mut JSString {
    unsafe { ToString(cx, val.into_raw()) }
}

pub fn to_rust_string(cx: *mut JSContext, val: HandleString) -> String {
    unsafe { JSStringToRustString(cx, val.into_raw()) }
}

pub fn is_compilable_unit(cx: *mut JSContext, handle: HandleObject, buffer: &str) -> bool {
    unsafe { Utf8IsCompilableUnit(cx, handle.into_raw(), buffer) }
}

pub fn context_options_ref<'a>(cx: *mut JSContext) -> Pin<&'a mut ContextOptions> {
    unsafe { raw_context_options_ref(cx) }
}
