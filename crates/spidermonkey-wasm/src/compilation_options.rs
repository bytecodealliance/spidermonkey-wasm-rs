use spidermonkey_wasm_sys::{
    jsffi::{CompileOptionsParams, JSContext, MakeOwningCompileOptions, OwningCompileOptions},
    UniquePtr,
};
use std::ops::Deref;

pub struct CompilationOptions {
    inner: UniquePtr<OwningCompileOptions>,
}

impl CompilationOptions {
    pub fn new(
        context: *mut JSContext,
        lineno: usize,
        force_full_parse: bool,
        file: String,
    ) -> Self {
        let opts = CompileOptionsParams {
            lineno,
            force_full_parse,
            file,
        };

        Self {
            inner: unsafe { MakeOwningCompileOptions(context, &opts) },
        }
    }
}

impl Deref for CompilationOptions {
    type Target = OwningCompileOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
