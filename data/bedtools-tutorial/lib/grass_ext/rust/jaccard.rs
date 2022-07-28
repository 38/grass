use grass_runtime::{
    algorithm::Components, 
    property::Tagged
};

let mut file_dep = [0;2];

let mut join_len = 0;
let mut union_len = 0;
let mut last_pos = 0;

for comp in input.components() {
    let (_, pos) = comp.position();
    let tag = comp.tag().unwrap() as usize;
    let len = pos - last_pos;
    
    if file_dep[0] > 0 && file_dep[1] > 0 {
        join_len += len;
    }

    if file_dep[0] > 0 || file_dep[1] > 0 {
        union_len += len;
    }
    
    if comp.is_open {
        file_dep[tag] += 1;
    } else {
        file_dep[tag] -= 1;
    }

    last_pos = pos;
}

println!("intersection\tunion\tjaccard");
println!("{}\t{}\t{}", join_len, union_len, join_len as f64 / union_len as f64);