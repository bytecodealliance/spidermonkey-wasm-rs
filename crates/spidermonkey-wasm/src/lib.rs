pub mod handle;
pub mod rooted;
pub mod runtime;

pub mod jsapi {
    pub use spidermonkey_wasm_sys::jsffi::*;
    pub use spidermonkey_wasm_sys::jsgc;
    pub use spidermonkey_wasm_sys::jsrealm;
}
