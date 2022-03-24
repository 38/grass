use rand::{prelude::ThreadRng, thread_rng, Rng};
use crate::{Genome, record::Bed3};


pub struct SortedRandomInterval {
    rng: ThreadRng,
    chrom_sizes: Vec<(&'static str, usize)>,
    regions: Vec<(usize, usize)>,
    chrom_idx: usize,
    flatten_region_begin: usize,
    flatten_region_end: usize,
    length_min: usize,
    length_max: usize,
    count: usize,
}

impl SortedRandomInterval {
    pub fn new(length_min: usize, length_max: usize, count: usize) -> SortedRandomInterval {
        let mut chrom_sizes = Genome::get_chrom_sizes();
        for idx in 1..chrom_sizes.len() {
            chrom_sizes[idx].1 += chrom_sizes[idx - 1].1;
        }
        let mut regions = Vec::new();
        let mut flatten_region_end = 0;
        for size in chrom_sizes.iter().map(|(_, size)| (*size).max(length_min) - length_min) {
            regions.push((flatten_region_end, flatten_region_end + size));
            flatten_region_end += size;
        }
        Self {
            rng: thread_rng(),
            regions,
            chrom_sizes,
            chrom_idx: 0,
            flatten_region_begin: 0,
            flatten_region_end,
            length_min,
            length_max,
            count,
        }
    }

    pub fn generate_next_interval(&mut self) -> Option<Bed3> {
        if self.count == 0 {
            return None;
        }
        let (beg, end) = self.generate_raw_interval();
        while self.chrom_idx < self.regions.len() && self.regions[self.chrom_idx].1 < beg {
            self.chrom_idx += 1;
        }
        if self.chrom_idx >= self.regions.len() {
            return None;
        }
        let chr_beg = beg - self.regions[self.chrom_idx].0;
        let chr_end = (end - self.regions[self.chrom_idx].0).min(self.chrom_sizes[self.chrom_idx].1);
        let chr_name = Genome::query_chr(self.chrom_sizes[self.chrom_idx].0);
        self.count -= 1;
        Some(Bed3 {
                chrom: chr_name,
                start: chr_beg as u32,
                end: chr_end as u32,
        })
    }

    fn generate_raw_interval(&mut self) -> (usize, usize) {
        let begin = self.generate_next_random_point(self.count);
        let end = self.rng.gen_range(begin + self.length_min..begin + self.length_max);
        (begin, end)
    }

    fn generate_next_random_point(&mut self, k : usize) -> usize {
        let linear_p : f64 = self.rng.gen_range(0.0..1.0);
        let adjusted = 1.0 - linear_p.powf(1.0 / k as f64);

        let mapped = self.flatten_region_begin as f64 + ((self.flatten_region_end - self.flatten_region_begin) as f64) * adjusted;

        let ret = mapped as usize;
        self.flatten_region_begin = ret;

        ret
    }
}

impl Iterator for SortedRandomInterval {
    type Item = Bed3;

    fn next(&mut self) -> Option<Self::Item> {
        self.generate_next_interval()
    }
}