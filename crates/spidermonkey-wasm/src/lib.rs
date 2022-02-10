pub mod runtime;
pub mod rooted;
pub mod handle;

pub mod jsapi {
    pub use spidermonkey_wasm_sys::jsffi::*;
    pub use spidermonkey_wasm_sys::jsrealm; 
    pub use spidermonkey_wasm_sys::jsgc; 
}
