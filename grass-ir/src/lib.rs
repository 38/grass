use std::collections::BTreeMap;

pub use field_expr::{
    BinaryParam, ComponentFieldRefParam, CondParam, ConstParam, ConstValue, FieldExpression,
    FieldRefParam, RecordRefParam, StringRepr, UnaryParam,
};
use serde::{Deserialize, Serialize};

use strum::AsRefStr;

mod field_expr;

#[derive(Serialize, Deserialize, Debug, Clone, AsRefStr)]
#[serde(tag = "opcode")]
pub enum GrassIR {
    /// Cast the inner data stream to a bed3 data stream
    CastToBed(CastToBedParam),
    /// Assign a label to a GRASS expression
    Let(LetBinding),
    /// Reference to an existing GRASS expression
    Ref(RefParam),
    /// Open a external data source
    Open(OpenParam),
    /// Write the result of GRASS expression to a file/file_no
    WriteFile(WriteFileParam),
    /// Modify a field for each record in a GRASS expression
    Alter(AlterParam),
    /// Filter records in a GRASS expression
    Filter(FilterParam),
    /// Merge any overlapped records in a GRASS expression
    Merge(MergeParam),
    /// Intersect two GRASS expression
    Intersection(IntersectParam),
    /// Customize the output format of records in a GRASS expression
    Format(FormatParam),
    /// Group the records in a GRASS expression into groups
    GroupBy(GroupByParam),

    AssumeSorted(AssumeSortedParam),

    InlineRust(InlineRustParam),

    LoadGenomeFile(LoadGenomeFileParam),

    SortedRandom(SortedRandomParam),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SortedRandomParam {
    pub count: usize,
    pub min_length: u32,
    pub max_length: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoadGenomeFileParam {
    File(String),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineRustParam {
    pub env: BTreeMap<String, GrassIR>,
    pub src: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefParam {
    /// The symbol we are referencing
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupByParam {
    /// The expression to group
    #[serde(rename = "inner")]
    pub expr: Box<GrassIR>,
    /// The list of key expressions for grouping
    pub keys: Vec<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatParam {
    /// The expression to be formatted
    #[serde(rename = "inner")]
    pub expr: Box<GrassIR>,
    /// The formatting string
    pub fmt_str: String,
    /// The value referred by the formatting string
    pub values: BTreeMap<String, FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IntersectFlavor {
    #[serde(rename = "inner")]
    Inner,
    #[serde(rename = "outer")]
    Outer,
    #[serde(rename = "left-outer")]
    LeftOuter,
    #[serde(rename = "right-outer")]
    RightOuter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntersectParam {
    /// The flavor of the insection operator
    pub flavor: IntersectFlavor,
    /// The left-hand-side operand
    pub lhs: Box<GrassIR>,
    /// The right-hand-side operand
    pub rhs: Box<GrassIR>,
    /// If we are using the sorted algorithm
    pub sorted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeParam {
    #[serde(rename = "inner")]
    pub input_expr: Box<GrassIR>,
    pub sorted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterParam {
    /// The original expression
    #[serde(rename = "inner")]
    pub input_expr: Box<GrassIR>,
    ///  The condition expression
    pub cond: FieldExpression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlterParam {
    /// The original expression
    #[serde(rename = "inner")]
    pub original_expr: Box<GrassIR>,
    /// The field name that we are going to modify
    pub field: String,
    /// The new value this field should assigned to
    pub value: FieldExpression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssumeSortedParam {
    pub inner: Box<GrassIR>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CastToBedParam {
    pub inner: Box<GrassIR>,
    pub num_of_fields: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputFormat {
    Bam,
    Bed,
    Cram,
    Vcf,
    Fasta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpenTarget {
    Path(String),
    FileNo(u32),
    CmdArg(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenParam {
    /// The path to the data source
    pub target: OpenTarget,
    /// What format of the input we are expecting
    pub format: InputFormat,
    /// How many field for each record
    pub num_of_fields: i32,
    /// If the input file is compressed
    pub compression: bool,
    /// If this file is known sorted
    pub sorted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WriteTarget {
    Path(String),
    FileNo(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WriteFileParam {
    /// The expression we want to write
    pub what: Box<GrassIR>,
    /// The target file or file number
    pub target: WriteTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LetBinding {
    /// The symbol of the value
    pub id: String,
    /// The actual expression that assigned to this symbol
    pub value: Box<GrassIR>,
}

#[cfg(test)]
mod test {
    use std::{collections::BTreeMap, error::Error};

    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    use crate::GrassIR;

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    #[serde(untagged)]
    enum JsonValue {
        String(String),
        Number(f64),
        Boolean(bool),
        List(Vec<JsonValue>),
        Object(BTreeMap<String, JsonValue>),
    }

    fn validate_object<'a, T: Serialize>(input: &str, obj: &'a T) {
        let input_dict: JsonValue = serde_json::from_str(input).unwrap();
        let obj_str = serde_json::to_string(obj).unwrap();
        let obj_dict: JsonValue = serde_json::from_str(&obj_str).unwrap();
        assert_eq!(obj_dict, input_dict);
    }

    macro_rules! parse_test {
        ($name: ident, $path : expr) => {
            #[test]
            fn $name() -> Result<(), Box<dyn Error>> {
                let input = include_str!($path);
                let data: GrassIR = from_str(input)?;
                validate_object(input, &data);
                Ok(())
            }
        };
    }
    parse_test!(parse_bam_to_bed, "../../data/ir/bam-to-bed.py.json");
    parse_test!(
        parse_expand_interval,
        "../../data/ir/expand-interval.py.json"
    );
    parse_test!(parse_filter, "../../data/ir/filter.py.json");
    parse_test!(parse_merge, "../../data/ir/merge.py.json");
    parse_test!(parse_slop, "../../data/ir/slop.py.json");
    parse_test!(
        parse_sorted_intersect_custom_format,
        "../../data/ir/sorted-intersect-custom-fmt.py.json"
    );
    parse_test!(
        parse_sorted_intersect_groupby,
        "../../data/ir/sorted-intersect-group.py.json"
    );
    parse_test!(
        parse_sorted_intersect_leftouter,
        "../../data/ir/sorted-intersect-leftouter.py.json"
    );
    parse_test!(
        parse_sorted_intersect_overlap_filter,
        "../../data/ir/sorted-intersect-overlap-filter.py.json"
    );
    parse_test!(
        parse_sorted_intersect,
        "../../data/ir/sorted-intersect.py.json"
    );
    parse_test!(parse_sorted_window, "../../data/ir/window.py.json");
}
