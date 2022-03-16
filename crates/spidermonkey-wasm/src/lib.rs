pub mod compilation_options;
pub mod handle;
pub mod js;
pub mod rooted;
pub mod runtime;
pub mod utf8_source;

pub use spidermonkey_wasm_sys::jsffi::{
    JSClass, JSContext, JSObject, OnNewGlobalHookOption, RealmOptions, WeakRefSpecifier,
};

pub use spidermonkey_wasm_sys::jsrealm::JSAutoRealm;

// Re-export low-level Rooted types for macro convenience
// and for GC callback definition
pub use spidermonkey_wasm_sys::jsgc::{
    JSGCReason, JSGCStatus, OnJSGCCallback, Rooted as RawRooted,
};
