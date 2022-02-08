use spidermonkey_wasm_sys::jsffi::{
    DefaultHeapMaxBytes, InitDefaultSelfHostedCode, JSContext, JSRuntime, JS_DestroyContext,
    JS_GetRuntime, JS_Init, JS_NewContext, JS_ShutDown, UseInternalJobQueues,
};
use std::ptr;

pub struct Runtime {
    context: *mut JSContext,
}

// This implementation doesn't reflect the entire parent-child
// relationship between runtimes. It assumes a single, top level runtime
// and a single context. This should be enough for the Wasm
// use case. This implementation can be expanded if necessary.
impl Default for Runtime {
    fn default() -> Self {
        assert!(JS_Init());

        let context: *mut JSContext =
            unsafe { JS_NewContext(DefaultHeapMaxBytes(), ptr::null_mut()) };

        unsafe {
            assert!(UseInternalJobQueues(context));
            assert!(InitDefaultSelfHostedCode(context));
        }

        Self { context }
    }
}

impl Runtime {
    pub fn cx(&self) -> *mut JSContext {
        self.context
    }

    pub fn rt(&self) -> *const JSRuntime {
        unsafe { JS_GetRuntime(self.context) }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JS_DestroyContext(self.context);
        }
        JS_ShutDown();
    }
}

#[cfg(test)]
mod tests {

    use super::Runtime;

    #[test]
    fn cx() {
        let rt = Runtime::default();
        assert!(!rt.cx().is_null());
    }

    #[test]
    fn rt() {
        let rt = Runtime::default();
        assert!(!rt.rt().is_null());
    }
}
