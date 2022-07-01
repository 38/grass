# grass

## What is Grass?

GRASS is a genomics data manipulation and analysis system powered by Rust programming language.

GRASS uses python as frontend language and defines a DSL(domain specific language) for genomics data analysis. The DSL then is transcompiled to Rust code and compiled to executable binary. The goal of GRASS is to provide a expressive and fast way to analyze genomics data.

## Installation

You can install pygrass from [PyPI](https://pypi.python.org/pypi/pygrass) and use GRASS.

```bash
pip install pygrass
```

## Usage by Examples

### 1. Intersection two sorted interval file and print the result to stdout

```python
    # simple_intersect.py

    import pygrass

    # Open two files
    file_a = pygrass.IntervalFile("file_a.bed")
    file_b = pygrass.IntervalFile("file_b.bed")

    # Intersect two files and print
    file_a.intersect(file_b).print_to_stdout()
```

You can simple run this python script and get the result. When you first time run this script,
you may experience a long waiting time due to the Rust source code compilation. But the binary artifact is then cached for later use. When you run similar query later, it will find the previously
built binary artifact and use it. (For example, you change the input file name, then GRASS will pick
the previously built binary artifact and use it.)

Although this looks like it's a python script, but this python code doesn't actually execute the 
operation. It just captures what operation you want to perform and create a intermediate representation
for this operation. To see the operation, you can run the following command:

```bash
# simple_intersect.py
GRASS_BACKEND_CLASS=pygrass.backend.DumpIR python3 simple_intersec.py
```

This will print the IR of the intersection operation. It may look like this:

```json
{
    "opcode": "WriteFile",
    "what": {
        "opcode": "Let",
        "id": "_grass_res_0",
        "value": {
            "opcode": "Intersection",
            "flavor": "inner",
            "lhs": {
                "opcode": "Let",
                "id": "_grass_res_1",
                "value": {
                    "opcode": "Open",
                    "target": {
                        "Path": {
                            "const_bag_key": 0
                        }
                    },
                    "format": "Bed",
                    "num_of_fields": 3,
                    "compression": false,
                    "sorted": true
                }
            },
            "rhs": {
                "opcode": "Let",
                "id": "_grass_res_2",
                "value": {
                    "opcode": "Open",
                    "target": {
                        "Path": {
                            "const_bag_key": 1
                        }
                    },
                    "format": "Bed",
                    "num_of_fields": 3,
                    "compression": false,
                    "sorted": true
                }
            },
            "sorted": true
        }
    },
    "target": 1
}
```

Note that no actual intersection is performed by Python interpreter.
Thus we are able to see intersection with pygrass runs as fast as native C/C++ code.

## 2. Chaining basic operations

One of the most powerful feature of GRASS is chainning basic operations.
Unlike you are chainning command line tools with pipes, in GRASS the chained operations are fused into
a single binary artifact which avoids huge performance overhead by repeatly serializing and deserializing data.

For example, you can shift the the intervals from file_a.bed and then intersect with file_b.bed.

```python
    # chaining_intersect.py
    import pygrass

    # Open two files
    file_a = pygrass.IntervalFile("file_a.bed")
    file_b = pygrass.IntervalFile("file_b.bed")

    # Extend intervals from file_a
    # Note that after the alteration, GRASS doesn't know if the intervals are sorted or not.
    # Thus, you need to explicitly set the sorted flag to true by calling `assume_sorted` method.
    extended_file_a = file_a.alter(start = pygrass.start - 50, end = pygrass.end - 50).assume_sorted()

    # Extend intervals from file_a.bed and intersect with file_b.bed
    extended_file_a.intersect(file_b).print_to_stdout()
```

This is equivalent to the following command with bedtools:

```bash
bedtools shift -s 50 file_a.bed | bedtools intersect -a - -b file_b.bed
```

Unlike bedtools invocation, pygrass actually generates a problem specific Rust program for the operation.
You can see the Rust program by running the following command:

```bash
# chaining_intersect.py
GRASS_BACKEND_CLASS=pygrass.backend.DumpRustCode python3 chaining_intersect.py
```

And this will print the Rust code.

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(unused_imports)]
use grass_runtime::const_bag::{ConstBagRef, ConstBagType};
const __CONST_BAG_VALUE_0: ConstBagRef<String> = ConstBagRef::<String>::new(0);
const __CONST_BAG_VALUE_1: ConstBagRef<f64> = ConstBagRef::<f64>::new(1);
const __CONST_BAG_VALUE_2: ConstBagRef<f64> = ConstBagRef::<f64>::new(2);
const __CONST_BAG_VALUE_3: ConstBagRef<String> = ConstBagRef::<String>::new(3);
fn grass_query_0(cmd_args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let _grass_query_temp_f5ad7a5bff714cd69a4983d46ed7e408 = {
        use grass_runtime::LineRecordStreamExt;
        use grass_runtime::algorithm::AssumeSorted;
        (std::fs::File::open(__CONST_BAG_VALUE_0.value())?)
            .into_record_iter::<grass_runtime::record::Bed3>()
            .assume_sorted()
    };
    let _grass_query_temp_b97606781e914e8c808dfe9119dab4a3 = {
        use grass_runtime::algorithm::AssumeSorted;
        let ret = _grass_query_temp_f5ad7a5bff714cd69a4983d46ed7e408.map(|mut item| {
            let new_value = Some(&item)
                .map(|_arg| {
                    ({
                        use grass_runtime::property::*;
                        _arg.start()
                    } as f64)
                        - (__CONST_BAG_VALUE_1.value())
                })
                .unwrap();
            item.set_start(new_value);
            item
        });
        ();
        ret
    };
    let _grass_query_temp_87eecafbfd1d47e9b5030a7e7d9fef8c = {
        use grass_runtime::algorithm::AssumeSorted;
        let ret = _grass_query_temp_b97606781e914e8c808dfe9119dab4a3.map(|mut item| {
            let new_value = Some(&item)
                .map(|_arg| {
                    ({
                        use grass_runtime::property::*;
                        _arg.end()
                    } as f64)
                        - (__CONST_BAG_VALUE_2.value())
                })
                .unwrap();
            item.set_end(new_value);
            item
        });
        ();
        ret
    };
    let _grass_query_temp_20606989ee8041d98cede5a20cb44419 = {
        use grass_runtime::algorithm::AssumeSorted;
        _grass_query_temp_87eecafbfd1d47e9b5030a7e7d9fef8c.assume_sorted()
    };
    let _grass_query_temp_516296693b3a483298ab5fa7d89828d3 = {
        use grass_runtime::LineRecordStreamExt;
        use grass_runtime::algorithm::AssumeSorted;
        (std::fs::File::open(__CONST_BAG_VALUE_3.value())?)
            .into_record_iter::<grass_runtime::record::Bed3>()
            .assume_sorted()
    };
    let _grass_query_temp_bcc301d7add847bca7ae45e58e71f33b = {
        use grass_runtime::algorithm::SortedIntersect;
        _grass_query_temp_20606989ee8041d98cede5a20cb44419
            .sorted_intersect(_grass_query_temp_516296693b3a483298ab5fa7d89828d3)
    };
    let _grass_query_temp_9fcad4cdbbe84dd392aed4ba2c15e049 = {
        #[cfg(unix)]
        use std::os::unix::io::FromRawFd;
        use std::io::Write;
        use grass_runtime::property::Serializable;
        let mut out_f = std::io::BufWriter::new(unsafe { std::fs::File::from_raw_fd(1i32) });
        for item in _grass_query_temp_bcc301d7add847bca7ae45e58e71f33b {
            item.dump(&mut out_f)?;
            out_f.write_all(b"\n")?;
        }
    };
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owned_cmd_args: Vec<_> = std::env::args().collect();
    let cmd_args: Vec<_> = owned_cmd_args.iter().map(|a| a.as_str()).collect();
    grass_query_0(&cmd_args)?;
    Ok(())
}
```
