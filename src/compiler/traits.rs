use crate::parser::ParseType;
use cranelift_codegen::isa::OwnedTargetIsa;
use std::fmt::Display;

pub trait CompilationType: ToString + Display + Sized + Clone + PartialEq {
    fn is_numeric(&self) -> bool;
    fn is_pointer(&self) -> bool;
    fn is_c_abi(&self) -> bool;
    fn into_c_abi(self) -> Self;

    fn size_bytes(&self, isa: &OwnedTargetIsa) -> u8;
    fn size_bits(&self, isa: &OwnedTargetIsa) -> u8;

    fn align_bytes(&self, isa: &OwnedTargetIsa) -> u8;
    fn align_bits(&self, isa: &OwnedTargetIsa) -> u8;

    fn inner(&self) -> Option<Self>;

    fn pseudo(&self) -> Option<Self>;

    fn cmp_eq(&self, other: &Self) -> bool;
}

pub trait TypeGenerator<T: CompilationType> {
    fn merge(&mut self, other: &Self);
    fn register_type(&mut self, name: String, ty: T);
    fn compile_type(&self, ty: &ParseType, isa: &OwnedTargetIsa);
}
