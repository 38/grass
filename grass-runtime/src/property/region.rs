use crate::ChrRef;


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

impl<T: Region> RegionCore for Option<T> {
    #[inline(always)]
    fn start(&self) -> u32 {
        self.as_ref().map_or(0, |what| what.start())
    }
    #[inline(always)]
    fn end(&self) -> u32 {
        self.as_ref().map_or(0, |what| what.end())
    }
    #[inline(always)]
    fn chrom(&self) -> ChrRef<'static> {
        self.as_ref().map_or(ChrRef::Dummy, |what| what.chrom())
    }
}

impl<'a, T: Region> RegionCore for &'a T {
    #[inline(always)]
    fn start(&self) -> u32 {
        T::start(*self)
    }
    #[inline(always)]
    fn end(&self) -> u32 {
        T::end(*self)
    }
    #[inline(always)]
    fn chrom(&self) -> ChrRef<'static> {
        T::chrom(*self)
    }
}

impl<A: Region, B: Region> RegionCore for (A, B) {
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