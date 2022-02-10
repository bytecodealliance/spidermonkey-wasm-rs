use crate::jsffi::{
    AutoRooterListHeads, GeckoProfilerThread, JSContext, JSObject, JSScript, JSString, Realm,
    Value, Zone,
};
use std::{cell::UnsafeCell, marker::PhantomData};
use std::{ffi::c_void, ptr};

// -- ROOTING

#[allow(non_snake_case)]
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

#[repr(C)]
#[derive(Debug)]
pub struct Rooted<T> {
    pub stack: *mut *mut Rooted<*mut c_void>,
    pub prev: *mut Rooted<*mut c_void>,
    pub ptr: T,
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
    pub unsafe fn init(&mut self, context: *mut JSContext, initial: T)
    where
        T: JSRootKind,
    {
        self.ptr = initial;
        let kind = T::root_kind() as usize;
        let rooting_context = context as *mut RootingContext;
        let stack: *mut *mut Rooted<*mut c_void> =
            &mut (*rooting_context).stackRoots_[kind] as *mut _ as *mut _;

        self.stack = stack;
        self.prev = *stack;
        *stack = self as *mut _ as usize as _;
    }

    pub unsafe fn remove_from_root_stack(&mut self) {
        assert!(*self.stack == self as *mut _ as usize as _);
        *self.stack = self.prev;
    }
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
        RootKind::Object
    }
}

impl JSRootKind for Value {
    fn root_kind() -> RootKind {
        RootKind::Value
    }
}

impl JSRootKind for *mut JSString {
    fn root_kind() -> RootKind {
        RootKind::String
    }
}

impl JSRootKind for *mut JSScript {
    fn root_kind() -> RootKind {
        RootKind::Script
    }
}

// HANDLE

#[repr(C)]
#[derive(Debug)]
pub struct Handle<T> {
    pub ptr: *const T,
    pub _marker: PhantomData<UnsafeCell<T>>,
}

#[repr(C)]
#[derive(Debug)]
pub struct MutableHandle<T> {
    pub ptr: *mut T,
    pub _marker: PhantomData<UnsafeCell<T>>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(i8)]
pub enum JSGCParamKey {
    JsgcMaxBytes = 0,
    JsgcMaxNurseryBytes = 2,
    JsgcBytes = 3,
    JsgcNumber = 4,
    JsgcIncrementalGcEnabled = 5,
    JsgcPerZoneGcEnabled = 6,
    JsgcUnusedChunks = 7,
    JsgcTotalChunks = 8,
    JsgcSliceTimeBudgetMs = 9,
    JsgcMarkStackLimit = 10,
    JsgcHighFrequencyTimeLimit = 11,
    JsgcSmallHeapSizeMax = 12,
    JsgcLargeHeapSizeMin = 13,
    JsgcHighFrequencySmallHeapGrowth = 14,
    JsgcHighFrequencyLargeHeapGrowth = 15,
    JsgcLowFrequencyHeapGrowth = 16,
    JsgcAllocationThreshold = 19,
    JsgcMinEmptyChunkCount = 21,
    JsgcMaxEmptyChunkCount = 22,
    JsgcCompactingEnabled = 23,
    JsgcSmallHeapIncrementalLimit = 25,
    JsgcLargeHeapIncrementalLimit = 26,
    JsgcNurseryFreeThresholdForIdleCollection = 27,
    JsgcPretenureThreshold = 28,
    JsgcPretenureGroupThreshold = 29,
    JsgcNurseryFreeThresholdForIdleCollectionPercent = 30,
    JsgcMinNurseryBytes = 31,
    JsgcMinLastDitchGcPeriod = 32,
    JsgcZoneAllocDelayKb = 33,
    JsgcNurseryBytes = 34,
    JsgcMallocThresholdBase = 35,
    JsgcIncrementalWeakmapEnabled = 37,
    JsgcChunkBytes = 38,
    JsgcHelperThreadRatio = 39,
    JsgcMaxHelperThreads = 40,
    JsgcHelperThreadCount = 41,
    JsgcPretenureStringThreshold = 42,
    JsgcStopPretenureStringThreshold = 43,
    JsgcMajorGcNumber = 44,
    JsgcMinorGcNumber = 45,
    JsgcNurseryTimeoutForIdleCollectionMs = 46,
    JsgcSystemPageSizeKb = 47,
    JsgcUrgentThresholdMb = 48,
}
