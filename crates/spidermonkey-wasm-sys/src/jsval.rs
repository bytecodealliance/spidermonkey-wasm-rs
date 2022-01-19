use cxx::{ExternType, type_id};

unsafe impl ExternType for Value {
    type Id = type_id!("JS::Value");
    type Kind = cxx::kind::Trivial;
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Value {
    pub asBits_: u64,
}