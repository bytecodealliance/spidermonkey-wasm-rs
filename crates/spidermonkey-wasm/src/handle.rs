use std::marker::PhantomData;

use spidermonkey_wasm_sys::{
    jsffi::{JSObject, JSScript, JSString, Value},
    jsgc::{Handle as RawHandle, MutableHandle as RawMutableHandle},
};

pub type HandleString<'a> = Handle<'a, *mut JSString>;
pub type HandleObject<'a> = Handle<'a, *mut JSObject>;
pub type HandleScript<'a> = Handle<'a, *mut JSScript>;
pub type HandleValue<'a> = Handle<'a, Value>;
pub type MutableHandleString<'a> = MutableHandle<'a, *mut JSString>;
pub type MutableHandleObject<'a> = MutableHandle<'a, *mut JSObject>;
pub type MutableHandleScript<'a> = MutableHandle<'a, *mut JSScript>;
pub type MutableHandleValue<'a> = MutableHandle<'a, Value>;

#[derive(Clone, Copy)]
pub struct Handle<'a, T: 'a> {
    ptr: &'a T,
}

impl<'a, T: 'a> Handle<'a, T> {
    pub fn new(ptr: &T) -> Handle<T> {
        Handle { ptr }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        *self.ptr
    }

    pub fn into_raw(self) -> RawHandle<T> {
        RawHandle {
            ptr: self.ptr as *const T,
            _marker: PhantomData,
        }
    }
}

pub struct MutableHandle<'a, T: 'a> {
    ptr: &'a mut T,
}

impl<'a, T: 'a> MutableHandle<'a, T> {
    pub fn new(ptr: &'a mut T) -> Self {
        MutableHandle { ptr }
    }

    pub fn into_raw(self) -> RawMutableHandle<T> {
        RawMutableHandle {
            ptr: self.ptr as *mut T,
            _marker: PhantomData,
        }
    }
}
