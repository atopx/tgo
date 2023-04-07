use std::fmt::{Display, Formatter};

/// # `TransformError` 错误定义
pub enum TransformError {
    /// `ParseError` 描述解析异常
    ParseError(String),
    /// `TransError` 描述转换异常
    TransError(String),
}

/// ### `TransformError` 的`Display`特征实现, 用于输出
impl Display for TransformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::ParseError(msg) => f.write_str(msg.as_str()),
            Self::TransError(msg) => f.write_str(msg.as_str()),
        };
    }
}

/// ### `parse_error` ParseError的包装器
pub fn parse_error(msg: &str) -> TransformError {
    return TransformError::ParseError(format!("transform parse error: {msg}"));
}

/// ### `trans_error` TransError的包装器
pub fn trans_error(msg: &str) -> TransformError {
    return TransformError::TransError(format!("transform trans error: {msg}"));
}