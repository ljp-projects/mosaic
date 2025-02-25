use crate::utils::Indirection;
use cranelift_codegen::ir::Value as RawValue;
use crate::compiler::identifiers::StatementIdentifier;

pub struct OwnershipTrace {
    /// Stores the name of the variable that owns the value, if any
    owner: OneOf<String, String, Statementidentifier>
}

pub enum CraneliftValue {
    Integer(i64, u8),
    UnsignedInetger(u64, u8),
    Float(f64, u8),
    Null,
    Boolean(bool),
    CPtr(RawValue),
    Slice(RawValue),
    CString,
    UnsignedCString,
    FunctionPtr(RawValue),
}

impl CraneliftValue {}
