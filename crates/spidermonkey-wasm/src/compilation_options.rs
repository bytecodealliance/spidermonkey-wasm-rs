use anyhow::{bail, Result};
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
    ) -> Result<Self> {
        let opts = CompileOptionsParams {
            lineno,
            force_full_parse,
            file,
        };

        let inner = unsafe { MakeOwningCompileOptions(context, &opts) };

        if inner.is_null() {
            bail!("Couldn't create compilation options")
        }

        Ok(Self { inner })
    }
}

impl Deref for CompilationOptions {
    type Target = OwningCompileOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
