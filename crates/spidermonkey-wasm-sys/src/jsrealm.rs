use cxx::{ExternType, type_id, kind};
use crate::jsffi::{self, JSContext, JSObject, EnterRealm, LeaveRealm};

unsafe impl ExternType for JSAutoRealm {
    type Id = type_id!("JSAutoRealm");
    type Kind = kind::Opaque;
}

#[repr(C)]
#[derive(Debug)]
pub struct JSAutoRealm {
    pub cx_: *mut jsffi::JSContext,
    pub oldRealm_: *mut jsffi::Realm,
}

impl JSAutoRealm {
    pub fn new(context: *mut JSContext, target: *mut JSObject) -> Self {
        Self {
            cx_: context,
            oldRealm_: unsafe { EnterRealm(context, target) },
        }
    }
}

impl Drop for JSAutoRealm {
    fn drop(&mut self) {
        unsafe {
            LeaveRealm(self.cx_, self.oldRealm_);
        }
    }
}
