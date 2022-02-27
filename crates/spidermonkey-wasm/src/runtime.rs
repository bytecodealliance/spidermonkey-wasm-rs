use crate::{
    handle::{HandleScript, MutableHandleValue},
    utf8_source::Utf8Source,
};
use spidermonkey_wasm_sys::{
    jsffi::{
        DefaultHeapMaxBytes, DisableIncrementalGC, InitDefaultSelfHostedCode, JSContext, JSRuntime,
        JSScript, JS_DestroyContext, JS_ExecuteScript, JS_GetRuntime, JS_Init, JS_NewContext,
        JS_SetGCParameter, JS_ShutDown, NonIncrementalGC, OwningCompileOptions, PrepareForFullGC,
        UseInternalJobQueues, Utf8SourceCompile, Utf8SourceEvaluate,
    },
    jsgc::{JSGCOptions, JSGCParamKey, JSGCReason},
};

use anyhow::{bail, Result};
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

    pub fn compile(
        &self,
        opts: &OwningCompileOptions,
        src: &mut Utf8Source,
    ) -> Result<*mut JSScript> {
        let ptr = unsafe { Utf8SourceCompile(self.context, opts, src.pin_mut()) };

        if ptr.is_null() {
            bail!("Script compilation failed");
        }

        Ok(ptr)
    }

    pub fn execute(&self, script_handle: HandleScript, rval: MutableHandleValue) -> Result<()> {
        let success =
            unsafe { JS_ExecuteScript(self.context, script_handle.into_raw(), rval.into_raw()) };

        if !success {
            bail!("Script execution failed");
        }

        Ok(())
    }

    pub fn eval(
        &self,
        opts: &OwningCompileOptions,
        src: &mut Utf8Source,
        rval: MutableHandleValue,
    ) -> Result<()> {
        let success =
            unsafe { Utf8SourceEvaluate(self.context, opts, src.pin_mut(), rval.into_raw()) };
        if !success {
            bail!("Eval failed");
        }
        Ok(())
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
