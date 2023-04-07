/// # `Shape` 数据结构定义
pub enum Shape {
    /// ### 特殊类型
    /// `Default` 无状态的默认结构
    Default,
    /// `None` 空, `nil` / `null` / `none`
    None,
    /// `Any` 范型，`interface` / `any` / `T`
    Any,

    /// ### 基础类型
    /// `Boolean` 布尔
    Boolean,
    /// `Number` 整数
    Number(NumberShape),
    /// `String` 字符串
    String(StringShape),

    /// ### 复合类型
    /// `Optional` 可选项, example: `number | null`
    Optional(Box<Shape>),
    /// `Array` 数组
    Array { t: Box<Shape> },
    /// `Tuple` 元组: 类型数组+长度
    Tuple(Vec<Shape>, usize),
    /// `Struct` 结构体: 用数组来描述
    Struct(Vec<(String, Shape)>),
    /// `Map` hash类型
    Map { ft: Box<Shape>, vt: Box<Shape> },
}

/// ### `NumberShape` 数值子类型
pub enum NumberShape {
    Default,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Float32,
    Float64,
}

/// ### `StringShape` 字符串子类型
pub enum StringShape {
    Default,
    Char,
    Varchar,
    Text,
}
