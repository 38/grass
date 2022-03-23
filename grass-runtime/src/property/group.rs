use std::io::{Result, Write};

use super::{Region, RegionCore, Serializable};
use crate::ChrRef;

pub trait GroupOps: RegionCore {
    fn component(&self, idx: usize) -> &dyn RegionCore;
    fn size(&self) -> usize;
}

pub trait DumpComponent: RegionCore {
    fn dump_component<W: Write>(&self, idx: usize, fp: W) -> Result<()>;
}

macro_rules! impl_intersection_trait {
    ($($t_name: ident),* => $($idx: tt),*) => {
        impl <$($t_name: Region + Serializable),*> DumpComponent for ($($t_name),*) {
            fn dump_component<W:Write>(&self, idx: usize, mut fp: W) -> Result<()> {
                match idx {
                    $($idx => self.$idx.dump(&mut fp),)*
                    _ => panic!("Index out of range")
                }
            }
        }
        impl <$($t_name: Region),*> GroupOps for ($($t_name),*) {
            fn component(&self, idx: usize) -> &dyn RegionCore {
                match idx {
                    $($idx => &self.$idx,)*
                    _ => panic!("Index out of range")
                }
            }
            fn size(&self) -> usize {
                $(let _ret = $idx;)*
                _ret + 1
            }
        }
    }
}

macro_rules! impl_with_region_for_tuple {
    (($($t_var:ident),*), ($($head:tt),*), $tail:tt) => {
       impl <$($t_var: Region),*> RegionCore for ($($t_var),*) {
           #[inline(always)]
           fn start(&self) -> u32 {
               if ($(&self . $head,)*).overlaps(&self.$tail) {
                   ($(&self . $head,)*).start().max(self.$tail.start())
               } else {
                   0
               }
           }
           #[inline(always)]
           fn end(&self) -> u32 {
               if ($(&self . $head,)*).overlaps(&self.$tail) {
                   ($(&self . $head,)*).end().min(self.$tail.end())
               } else {
                   0
               }
           }
           #[inline(always)]
           fn chrom(&self) -> ChrRef<'static> {
               self.0.chrom()
           }
       }
       impl_intersection_trait!($($t_var),* => $($head,)* $tail);
    };
}

impl_intersection_trait!(A, B => 0, 1);
impl_with_region_for_tuple!((A, B, C), (0, 1), 2);
impl_with_region_for_tuple!((A, B, C, D), (0, 1, 2), 3);
impl_with_region_for_tuple!((A, B, C, D, E), (0, 1, 2, 3), 4);
impl_with_region_for_tuple!((A, B, C, D, E, F), (0, 1, 2, 3, 4), 5);
impl_with_region_for_tuple!((A, B, C, D, E, F, G), (0, 1, 2, 3, 4, 5), 6);
impl_with_region_for_tuple!((A, B, C, D, E, F, G, H), (0, 1, 2, 3, 4, 5, 6), 7);
impl_with_region_for_tuple!((A, B, C, D, E, F, G, H, I), (0, 1, 2, 3, 4, 5, 6, 7), 8);
impl_with_region_for_tuple!(
    (A, B, C, D, E, F, G, H, I, J),
    (0, 1, 2, 3, 4, 5, 6, 7, 8),
    9
);
impl_with_region_for_tuple!(
    (A, B, C, D, E, F, G, H, I, J, K),
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
    10
);
