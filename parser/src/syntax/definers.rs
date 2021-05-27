use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct FunctionType {
    pub complete: bool,
    pub params: Vec<DefinerCollecting>,
    pub returning: Box<DefinerCollecting>,
    pub return_typed: bool,
    pub return_keyword: i8,
    pub parameter_collected: bool,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct CloakType {
    pub complete: bool,
    pub rtype: Vec<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct ArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
    pub len: crate::syntax::types::Types,
    pub at_comma: bool,
    pub typed: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct DynamicArrayType {
    pub complete: bool,
    pub rtype: Box<DefinerCollecting>,
    pub bracket_inserted: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    DynamicArray(DynamicArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
    Dynamic,
}

impl Default for DefinerCollecting {
    fn default() -> Self {
        DefinerCollecting::Generic(GenericType::default())
    }
}

impl DefinerCollecting {
    pub fn is_type_empty(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => !data.complete,
            DefinerCollecting::DynamicArray(data) => !data.complete,
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
        }
    }

    pub fn is_definer_complete(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => data.complete,
            DefinerCollecting::DynamicArray(data) => data.complete,
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true,
        }
    }

    pub fn raw_name(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_string(),
            DefinerCollecting::DynamicArray(_) => "dynamic_array".to_string(),
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string(),
        }
    }
}
