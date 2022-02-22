use spidermonkey_wasm_sys::{
    jsffi::{
        DefaultHeapMaxBytes, DisableIncrementalGC, InitDefaultSelfHostedCode, JSContext, JSRuntime,
        JS_DestroyContext, JS_GetRuntime, JS_Init, JS_NewContext, JS_SetGCParameter, JS_ShutDown,
        NonIncrementalGC, PrepareForFullGC, UseInternalJobQueues,
    },
    jsgc::{JSGCOptions, JSGCParamKey, JSGCReason},
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

    // TODO: Investigate if there's a need for
    // `AutoDisableGenerationalGC`; according to
    // the class' documentation, generational
    // GC is disabled by default (ref
    // https://searchfox.org/mozilla-central/source/js/public/GCAPI.h#964). Unless `--enable-gcgenerational`
    // is passed. which is not the case (ref
    // https://github.com/bytecodealliance/spidermonkey-wasm-build/blob/main/mozconfigs/release)
    pub fn disable_incremental_gc(&self) {
        unsafe { DisableIncrementalGC(self.context) }
    }

    pub fn set_gc_parameter(&self, key: JSGCParamKey, val: u32) {
        unsafe {
            JS_SetGCParameter(self.context, key, val);
        }
    }

    pub fn prepare_for_full_gc(&self) {
        unsafe { PrepareForFullGC(self.context) };
    }

    pub fn non_incremental_gc(&self, opts: JSGCOptions, reason: JSGCReason) {
        unsafe { NonIncrementalGC(self.context, opts, reason) };
    }

    // TODO(@saulecabrera) Add api to set gc callback
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
