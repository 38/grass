use std::io::{Result, Write};
use crate::ChrRef;

pub trait Parsable: Sized {
    fn parse(s: &str) -> Option<(Self, usize)>;
}

pub trait Serializable {
    fn dump<W: Write>(&self, fp: W) -> Result<()>;
}

pub trait RegionCore {
    fn start(&self) -> u32;
    fn end(&self) -> u32;
    fn chrom(&self) -> ChrRef<'static>;

    #[inline(always)]
    fn empty(&self) -> bool {
        self.end() <= self.start()
    }

    #[inline(always)]
    fn length(&self) -> u32 {
        self.end().max(self.start()) - self.start()
    }
}

pub trait Region: RegionCore {
    #[inline(always)]
    fn overlaps(&self, b: &impl Region) -> bool {
        let a = self;
        if a.chrom() != b.chrom() {
            return false;
        }

        !(a.end() <= b.start() || b.end() <= a.start())
    }
}

impl<T: RegionCore> Region for T {}

impl<'a, T: Region> RegionCore for &'a T {
    fn start(&self) -> u32 {
        T::start(*self)
    }
    fn end(&self) -> u32 {
        T::end(*self)
    }
    fn chrom(&self) -> ChrRef<'static> {
        T::chrom(*self)
    }
}

impl<A: Region, B: Region> RegionCore for (A, B)
{
    #[inline(always)]
    fn start(&self) -> u32 {
        if self.0.overlaps(&self.1) {
            self.0.start().max(self.1.start())
        } else {
            0
        }
    }

    #[inline(always)]
    fn end(&self) -> u32 {
        if self.0.overlaps(&self.1) {
            self.0.end().min(self.1.end())
        } else {
            0
        }
    }

    #[inline(always)]
    fn chrom(&self) -> ChrRef<'static> {
        self.0.chrom()
    }
}

pub trait IntersectOps: RegionCore {
    fn component(&self, idx: usize) -> &dyn RegionCore;
    fn size(&self) -> usize;
}

macro_rules! impl_intersection_trait {
    ($($t_name: ident),* => $($idx: tt),*) => {
        impl <$($t_name: Region),*> IntersectOps for ($($t_name),*) {
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

impl_intersection_trait!(A, B => 0, 1);

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

impl_with_region_for_tuple!((A, B, C), (0, 1), 2);
impl_with_region_for_tuple!((A, B, C, D), (0, 1, 2), 3);
impl_with_region_for_tuple!((A, B, C, D, E), (0, 1, 2, 3), 4);
impl_with_region_for_tuple!((A, B, C, D, E, F), (0, 1, 2, 3, 4), 5);
impl_with_region_for_tuple!((A, B, C, D, E, F, G), (0, 1, 2, 3, 4, 5), 6);
impl_with_region_for_tuple!((A, B, C, D, E, F, G, H), (0, 1, 2, 3, 4, 5, 6), 7);

pub trait WithName {
    fn name(&self) -> &str;
}

pub trait Scored<T : Sized> {
    fn score(&self) -> T;
}

pub enum Strand {
    Negative,
    Positive,
    Unknown,
}

pub trait Stranded {
    fn strand(&self) -> Strand {
        Strand::Unknown
    }
}

impl<A: Serializable, B: Serializable> Serializable for (A, B) {
    fn dump<W: Write>(&self, mut fp: W) -> Result<()> {
        self.0.dump(&mut fp)?;
        write!(fp, "\t|\t")?;
        self.1.dump(&mut fp)
    }
}

pub enum Nuclide {
    A,
    T,
    C,
    G,
    U,
    N,
}

pub trait WithSequence {
    type RangeType: IntoIterator<Item = Nuclide>;
    fn at(&self, offset: usize) -> Nuclide;
    fn range(&self, from: usize, to: usize) -> Self::RangeType;
}
