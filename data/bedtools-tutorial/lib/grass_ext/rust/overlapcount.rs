use grass_runtime::algorithm::{Components, TagAssignmentExt};
use grass_runtime::property::{Named, RegionCore, Tagged}; 
let mut cnt = (0, 0);
let mut active_a = std::collections::HashMap::<usize, usize>::new();
input.components().filter_map(move |comp| {
     if comp.tag() == Some(1) {
         if comp.is_open {
             cnt.0 += 1;
         } else {
             cnt.1 += 1;
         }
     } else {
         if comp.is_open {
             active_a.insert(comp.index, cnt.1);
         } else {
             let count = cnt.0 - active_a.remove(&comp.index).unwrap();
             return Some(comp.value.with_tag(count))
         }
     }
     None
})
