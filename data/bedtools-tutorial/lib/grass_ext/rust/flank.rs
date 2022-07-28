use grass_runtime::property::*;

let before_bases = before_bases as u32;
let after_bases = after_bases as u32;

input.flat_map(move |item| {
    // Create the interval before the original interval
    let mut before = item.clone();
    before.start = item.start().max(before_bases) - before_bases;
    before.end = item.start();

    // Create the interval after the original interval
    let mut after = item;
    after.start = after.end();
    after.end = after.end() + after_bases;

    // Chain the interval and return it
    std::iter::once(before).chain(std::iter::once(after))
})