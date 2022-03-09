use crate::handle::{Handle, MutableHandle};
use spidermonkey_wasm_sys::{
    jsffi::{JSContext, JSObject, JSScript, JSString, Value},
    jsgc::{JSRootKind, Rooted as RawRooted},
};
use std::pin::Pin;

pub type RootedValue<'a> = Rooted<'a, Value>;
pub type RootedObject<'a> = Rooted<'a, *mut JSObject>;
pub type RootedScript<'a> = Rooted<'a, *mut JSScript>;
pub type RootedString<'a> = Rooted<'a, *mut JSString>;

/// Helper to root values on the stack.
///
/// Inspired by: https://github.com/servo/rust-mozjs/blob/master/src/rust.rs#L546;
/// with the difference that this implementation allows rooting multiple values at once.
///
/// # Usage
///
///     root!(with(context);
///         let undefined_value = jsapi::UndefinedValue();
///         let other_vaue = jsapi::UndefinedValue();
///     );
///
#[macro_export]
macro_rules! root {
    (with($cx:expr); $(let $v:ident = $init:expr;)*) => { $(
        let mut $v = $crate::RawRooted::default();
        let $v = $crate::rooted::Rooted::new($cx, &mut $v, $init);
    )*};

    (with($cx:expr); $(let mut $v:ident = $init:expr;)*) => { $(
        let mut $v = $crate::RawRooted::default();
        let mut $v = $crate::rooted::Rooted::new($cx, &mut $v, $init);
    )*};
}

pub struct Rooted<'a, T: 'a + JSRootKind> {
    root: Pin<&'a mut RawRooted<T>>,
}

impl<'a, T: 'a + JSRootKind> Rooted<'a, T> {
    pub fn new(context: *mut JSContext, root: &'a mut RawRooted<T>, initial: T) -> Self {
        unsafe { root.init(context, initial) };

        Self {
            root: unsafe { Pin::new_unchecked(root) },
        }
    }

    pub fn handle(&self) -> Handle<T> {
        Handle::new(&self.root.ptr)
    }

    pub fn mut_handle(&mut self) -> MutableHandle<T> {
        let mut_pin = self.root.as_mut();
        let raw_rooted = unsafe { mut_pin.get_unchecked_mut() };

        MutableHandle::new(&mut raw_rooted.ptr)
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        self.root.ptr
    }
}

impl<'a, T: 'a + JSRootKind> Drop for Rooted<'a, T> {
    fn drop(&mut self) {
        inner_drop(self.root.as_mut());

        fn inner_drop<'a, T: 'a + JSRootKind>(this: Pin<&'a mut RawRooted<T>>) {
            let raw_root = unsafe { this.get_unchecked_mut() };
            unsafe { raw_root.remove_from_root_stack() };
        }
    }
}
