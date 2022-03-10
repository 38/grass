use std::collections::HashMap;

use field_expr::FieldExpression;
use serde::{Serialize, Deserialize};

mod field_expr;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "opcode")]
pub enum GrassIR {
    /// Cast the inner data stream to a bed3 data stream
    CastToBed3(CastToBed3Param),
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefParam {
    /// The symbol we are referencing
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupByParam {
    /// The expression to group
    #[serde(rename = "inner")]
    expr: Box<GrassIR>,
    /// The list of key expressions for grouping
    keys: Vec<FieldExpression>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormatParam {
    /// The expression to be formatted
    #[serde(rename = "inner")]
    expr: Box<GrassIR>,
    /// The formatting string
    fmt_str: String,
    /// The value referred by the formatting string
    values: HashMap<String, FieldExpression>,
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
    flavor: IntersectFlavor,
    /// The left-hand-side operand
    lhs: Box<GrassIR>,
    /// The right-hand-side operand
    rhs: Box<GrassIR>,
    /// If we are using the sorted algorithm
    sorted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeParam {
    #[serde(rename = "inner")]
    input_expr: Box<GrassIR>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilterParam {
    /// The original expression
    #[serde(rename = "inner")]
    input_expr: Box<GrassIR>,
    ///  The condition expression
    cond: FieldExpression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlterParam {
    /// The original expression
    #[serde(rename = "inner")]
    original_expr: Box<GrassIR>,
    /// The field name that we are going to modify
    field: String,
    /// The new value this field should assigned to
    value: FieldExpression,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  CastToBed3Param {
    inner: Box<GrassIR>,
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
pub struct OpenParam {
    /// The path to the data source
    pub path: String,
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
    use std::error::Error;

    use serde_json::from_str;

    use crate::GrassIR;
    #[test]
    fn parse_bam_to_bed() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/bam-to-bed.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_expand_interval() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/expand-interval.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
   
    #[test]
    fn parse_filter() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/filter.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_merge() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/merge.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_slop() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/slop.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_intersect_custom_format() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/sorted-intersect-custom-fmt.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_intersect_groupby() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/sorted-intersect-group.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_intersect_leftouter() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/sorted-intersect-leftouter.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_intersect_overlap_filter() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/sorted-intersect-overlap-filter.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_intersect() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/sorted-intersect.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
    
    #[test]
    fn parse_sorted_window() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../../data/ir/window.py.json");
        let _data : GrassIR = from_str(input)?;
        Ok(())
    }
}
