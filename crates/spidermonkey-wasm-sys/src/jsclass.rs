#[repr(C)]
#[repr(align(8))]
pub struct JSClass {
    pub name: *const ::std::os::raw::c_char,
    pub flags: u32,
    pub c_ops: *const crate::jsffi::JSClassOps,
    pub spec: *const crate::jsffi::ClassSpec,
    pub ext: *const crate::jsffi::ClassExtension,
    pub o_ops: *const crate::jsffi::ObjectOps,
}
