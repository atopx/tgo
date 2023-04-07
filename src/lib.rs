mod interface;
mod shape;
mod error;
mod schema;

use schema::Schema;


/// `Transform` 转换器定义
pub struct Transform {
    parser: Box<dyn ITransform>,
    transfer: Box<dyn ITransform>,
}

/// `Transform` 转换器实现
impl Transform {
    /// ### 实例化`Transform`, 并预加载转换器, 客户端在同类型下可复用对象
    pub fn new(on: Schema, to: Schema) -> Self {
        return Self {
            parser: Self::transform(&on),
            transfer: Self::transform(&to),
        };
    }

    /// ### `goto` 执行数据转换, 支持异步或多线程
    pub fn goto(&self, src: &str) -> Result<String, error::TransformError> {
        let shapes = self.parser.on(src)?;
        return self.transfer.to(shapes);
    }

    /// ### `transform` 构建转换器对象
    fn transform(t: &Schema) -> Box<dyn ITransform> {
        return match t {
            Schema::Json => Box::new(interface::json::Transform {}),
            Schema::Yaml => Box::new(interface::yaml::Transform {}),
            Schema::Toml => Box::new(interface::toml::Transform {}),
            Schema::Sql => Box::new(interface::sql::Transform {}),
            Schema::Xml => Box::new(interface::xml::Transform {}),
            Schema::Protobuf => Box::new(interface::protobuf::Transform {}),
            Schema::Golang => Box::new(interface::golang::Transform {}),
            Schema::Rust => Box::new(interface::rust::Transform {}),
            Schema::Java => Box::new(interface::java::Transform {}),
            Schema::Python => Box::new(interface::python::Transform {}),
            Schema::Typescript => Box::new(interface::typescript::Transform {}),
        };
    }
}


/// # `ITransform`转换器抽象: `source` -> `vec<shape>` -> `result`
trait ITransform {
    /// ### `parser` 从原始数据转换为中继数据
    fn on(&self, value: &str) -> Result<Vec<shape::Shape>, error::TransformError>;

    /// ### `transfer` 从中继数据转换为目标数据
    fn to(&self, shapes: Vec<shape::Shape>) -> Result<String, error::TransformError>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let transform = Transform::new(Schema::Golang, Schema::Rust);
        let result = transform.goto("123123");
        match result {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}
