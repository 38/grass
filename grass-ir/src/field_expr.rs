use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "opcode")]
pub enum FieldExpression {
    And(BinaryParam),
    Or(BinaryParam),
    Xor(BinaryParam),
    Not(UnaryParam),
    Add(BinaryParam),
    Sub(BinaryParam),
    Mul(BinaryParam),
    Div(BinaryParam),
    Mod(BinaryParam),
    Eq(BinaryParam),
    Ne(BinaryParam),
    LessThan(BinaryParam),
    GreaterThan(BinaryParam),
    LessEqualThan(BinaryParam),
    GreaterEqualThan(BinaryParam),
    RightShift(BinaryParam),
    LeftShift(BinaryParam),
    Neg(UnaryParam),
    Cond(CondParam),
    FieldRef(FieldRefParam),
    NumberOfComponents,
    ComponentFieldRef(ComponentFieldRefParam),
    ConstValue(ConstParam),
    FullRecordRef,
    RecordRef(RecordRefParam),
    StringRepr(StringRepr),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordRefParam {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ConstValue {
    Str(String),
    Number(i64),
    Float(f64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstParam {
    pub value: ConstValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnaryParam {
    pub operand: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryParam {
    pub lhs: Box<FieldExpression>,
    pub rhs: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CondParam {
    pub cond: Box<FieldExpression>,
    pub then: Box<FieldExpression>,
    pub elze: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringRepr {
    pub value: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldRefParam {
    pub field: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentFieldRefParam {
    pub target: i32,
    pub field_name: String,
}
