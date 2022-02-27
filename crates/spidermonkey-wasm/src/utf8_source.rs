use anyhow::{bail, Result};
use spidermonkey_wasm_sys::{
    jsffi::{JSContext, MakeUtf8UnitSourceText, SourceOwnership, Utf8UnitSourceText},
    UniquePtr,
};
use std::pin::Pin;

pub struct Utf8Source {
    inner: UniquePtr<Utf8UnitSourceText>,
}

impl Utf8Source {
    pub fn new(context: *mut JSContext, src: &str) -> Result<Self> {
        let inner =
            unsafe { MakeUtf8UnitSourceText(context, src, src.len(), SourceOwnership::Borrowed) };

        if inner.is_null() {
            bail!("Could not initialize Utf8Source");
        }

        Ok(Self { inner })
    }

    pub fn pin_mut(&mut self) -> Pin<&mut Utf8UnitSourceText> {
        self.inner.pin_mut()
    }
}
