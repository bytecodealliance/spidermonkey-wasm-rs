use std::marker::PhantomData;

use spidermonkey_wasm_sys::jsgc::{Handle as RawHandle, MutableHandle as RawMutableHandle};

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

    pub fn into_raw(&self) -> RawHandle<T> {
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

    pub fn into_raw(&mut self) -> RawMutableHandle<T> {
        RawMutableHandle {
            ptr: self.ptr as *mut T,
            _marker: PhantomData,
        }
    }
}
