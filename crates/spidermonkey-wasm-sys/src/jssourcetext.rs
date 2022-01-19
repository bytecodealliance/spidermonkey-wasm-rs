use cxx::{ExternType, type_id, kind};
use crate::jsffi::{Utf8UnitSourceText, U16SourceText};

unsafe impl ExternType for Utf8UnitSourceText {
    type Id = type_id!("Utf8UnitSourceText");
    type Kind = kind::Opaque;
}

unsafe impl ExternType for U16SourceText {
    type Id = type_id!("U16SourceText");
    type Kind = kind::Opaque;
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SourceText<Unit> {
    pub units_: *const Unit,
    pub length_: u32,
    pub ownsUnits_: bool,
}