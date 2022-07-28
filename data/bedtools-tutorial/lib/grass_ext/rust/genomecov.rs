use grass_runtime::{algorithm::Components, Genome, ChrRef};
type Histogram = std::collections::BTreeMap<usize, usize>;

// Printing the histogram for either genome-wide or per-chromosome
fn print_histogram(chr: Option<&ChrRef<'static>>, stat: &Histogram) {
    let genome_size:usize = Genome::get_chrom_sizes().into_iter().map(|s| s.1).sum();
    let chr_size = chr.map_or(genome_size, |chr| chr.get_chr_size().unwrap_or(0));
    let chr_name = chr.map_or("genome", |chr| chr.get_chr_name());
    let zero_count = chr_size - stat.values().sum::<usize>();
    for (k, &v) in std::iter::once((&0, &zero_count)).chain(stat.into_iter()) {
        if v > 0 {
            println!("{}\t{}\t{}\t{}\t{}", chr_name, k, v, chr_size, v as f64 / chr_size as f64);
        }
    }
}

// Genome-wide histogram
let mut genome_wide = Histogram::new();
// Chromosome wide histogram
let mut chr_wide = Histogram::new();

// Current scanning states
let mut cur_chr = Genome::get_chr_by_id(0).unwrap();
let mut cur_pos = 0;
let mut cur_dep = 0;

for comp in input.components() {
    let (chr, pos) = comp.position();
    while cur_chr != chr {
        print_histogram(Some(&cur_chr), &chr_wide);
        cur_pos = 0;
        cur_chr = cur_chr.next_chrom().unwrap();
        chr_wide.clear();
    }
    if cur_dep > 0 {
        let size = (pos - cur_pos) as usize;
        *chr_wide.entry(cur_dep).or_default() += size;
        *genome_wide.entry(cur_dep).or_default() += size;
    }
    cur_dep = comp.depth();
    cur_pos = pos;
}
print_histogram(Some(&cur_chr), &chr_wide);
print_histogram(None, &genome_wide);
