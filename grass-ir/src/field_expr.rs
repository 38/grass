use serde::{Serialize, Deserialize};

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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordRefParam {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ConstValue{
    Str(String),
    Number(i64),
    Float(f64)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstParam {
    value: ConstValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnaryParam {
    operand: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinaryParam {
    lhs: Box<FieldExpression>,
    rhs: Box<FieldExpression>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CondParam {
    cond: Box<FieldExpression>,
    then: Box<FieldExpression>,
    elze: Box<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldRefParam {
    field: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentFieldRefParam {
    target: i32,
    field_name: String,
}

