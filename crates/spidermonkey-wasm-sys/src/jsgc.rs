use crate::jsffi::{
    RootedObject, JSContext, JSObject,
    AutoRooterListHeads, GeckoProfilerThread, Realm, Zone
};
use std::{ffi::c_void, ptr};
use cxx::{type_id, ExternType};

unsafe impl ExternType for RootingContext {
    type Id = type_id!("RootingContext");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
pub struct RootingContext {
    pub stackRoots_: [u32; 15usize],
    pub autoGCRooters_: AutoRooterListHeads,
    pub geckoProfiler_: GeckoProfilerThread,
    pub realm_: *mut Realm,
    pub zone_: *mut Zone,
    pub nativeStackLimit: [usize; 3usize],
    pub wasiRecursionDepth: u32,
}

unsafe impl ExternType for RootedObject {
    type Id = type_id!("RootedObject");
    type Kind = cxx::kind::Opaque;
}

#[repr(C)]
#[derive(Debug)]
pub struct Rooted<T> {
    pub stack: *mut *mut Rooted<*mut c_void>,
    pub prev: *mut Rooted<*mut c_void>,
    pub ptr: T
}

impl<T> Default for Rooted<T> {
    fn default() -> Self {
        Self {
            stack: ptr::null_mut(),
            prev: ptr::null_mut(),
            ptr: unsafe { std::mem::zeroed() },
        }
    }
}

impl<T> Rooted<T> {
    pub fn new(context: *mut JSContext, ptr: T) -> Self where T: JSRootKind {
        let mut rooted = Self::default();
        rooted.root(context, ptr);
        rooted
    }

    fn root(&mut self, context: *mut JSContext, ptr: T) where T: JSRootKind {
        let kind = T::root_kind() as usize;
        let rooting_context = context as *mut RootingContext;
        let root_stack: *mut *mut Rooted<*mut c_void> = unsafe { &mut (*rooting_context).stackRoots_[kind] as *mut _ as *mut _ };

        self.stack = root_stack;
        unsafe {
            self.ptr = ptr;
            self.prev = *root_stack;
            *root_stack = self as *mut _ as usize as _;
        }
    }

    fn remove_from_root_stack(&mut self) {
        unsafe {
            assert!(*self.stack == self as *mut _ as usize as _);
            *self.stack = self.prev;
        }
    }
}

impl<T> Drop for Rooted<T> {
    fn drop(&mut self) {
        self.remove_from_root_stack();
    }
}

unsafe impl ExternType for RootKind {
    type Id = type_id!("JS::RootKind");
    type Kind = cxx::kind::Trivial;
}

#[repr(i8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RootKind {
    BaseShape = 0,
    JitCode = 1,
    Scope = 2,
    Object = 3,
    Script = 4,
    Shape = 5,
    String = 6,
    Symbol = 7,
    BigInt = 8,
    RegExpShared = 9,
    GetterSetter = 10,
    PropMap = 11,
    Id = 12,
    Value = 13,
    Traceable = 14,
    Limit = 15,
}

pub trait JSRootKind {
    fn root_kind() -> RootKind;
}

impl JSRootKind for *mut JSObject {
    fn root_kind() -> RootKind {
        return RootKind::Object;
    }
}


