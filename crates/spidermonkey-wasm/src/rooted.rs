use spidermonkey_wasm_sys::{jsgc::{Rooted as RawRooted, JSRootKind}, jsffi::JSContext};
use crate::handle::{Handle, MutableHandle};
use std::pin::Pin;

macro_rules! root {
    () => {
        
    };
}

// root!(

pub struct Rooted<'a, T: 'a + JSRootKind> {
    root: Pin<&'a mut RawRooted<T>>,
}

impl<'a, T: 'a + JSRootKind> Rooted<'a, T> {
     pub fn new(context: *mut JSContext, root: &'a mut RawRooted<T>, initial: T) -> Self {
         unsafe { root.init(context, initial) };

         Self { root: unsafe { Pin::new_unchecked(root) } }
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
        where T: Copy{
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
