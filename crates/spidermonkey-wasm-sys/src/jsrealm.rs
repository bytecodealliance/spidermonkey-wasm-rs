use crate::jsffi::{self, EnterRealm, JSContext, JSObject, LeaveRealm};

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
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
