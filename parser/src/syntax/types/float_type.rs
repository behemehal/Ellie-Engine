use alloc::format;
use alloc::string::{String, ToString};
use core::any::Any;
use core::any::TypeId;
use enum_as_inner::EnumAsInner;
use serde::Serialize;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize)]
pub enum FloatTypes {
    F32,
    F64,
}

impl Default for FloatTypes {
    fn default() -> Self {
        FloatTypes::F32
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, EnumAsInner)]
pub enum FloatSize {
    F32(f32),
    F64(f64),
}

impl FloatSize {
    pub fn get_type(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(bracket_offset.., "");
        q
    }
}

impl Default for FloatSize {
    fn default() -> Self {
        FloatSize::F64(0.0)
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct FloatType {
    pub value: FloatSize,
    pub rtype: FloatTypes,
    pub raw: String,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct FloatTypeCollector {
    pub data: FloatType,
    pub base: String,
    pub point: String,
    pub at_point: bool,
    pub complete: bool,
}

impl FloatTypeCollector {
    pub fn collect(&self) -> String {
        (self.base.to_string() + &(".".to_string())) + &self.point
    }

    pub fn build<T: Any>(raw: T) -> FloatType {
        if TypeId::of::<T>() == TypeId::of::<f32>() {
            FloatType {
                value: FloatSize::F32(*(&raw as &dyn Any).downcast_ref::<f32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else if TypeId::of::<T>() == TypeId::of::<f64>() {
            FloatType {
                value: FloatSize::F64(*(&raw as &dyn Any).downcast_ref::<f64>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        } else {
            FloatType {
                value: FloatSize::F32(*(&raw as &dyn Any).downcast_ref::<f32>().unwrap()),
                raw: (*(&raw as &dyn Any).downcast_ref::<String>().unwrap()).clone(),
                ..Default::default()
            }
        }
    }
}
