use crate::shape;
use crate::error::{parse_error, trans_error, TransformError};
use crate::ITransform as Base;

pub struct Transform;

impl Base for Transform {
    // 从原始数据转换为中继数据
    fn on(&self, value: &str) -> Result<Vec<shape::Shape>, TransformError> {
        return Err(parse_error("unimplemented"));
    }

    // 从中继数据转换为目标数据
    fn to(&self, shapes: Vec<shape::Shape>) -> Result<String, TransformError> {
        return Err(trans_error("unimplemented"));
    }
}