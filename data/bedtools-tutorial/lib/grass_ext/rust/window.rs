use grass_runtime::{
    Genome, 
    Itertools, 
    record::Bed3, 
    algorithm::AssumeSorted
};

let bin_size = bin_size as u32;

Genome::get_chrom_sizes().into_iter()
    .flat_map(move |(name, chr_size)| {
        let chrom = Genome::query_chr(name);
        let chr_size = chr_size as u32;
        (0..chr_size).step(bin_size as usize).map(move |start| Bed3 {
            chrom,
            start,
            end: (start + bin_size).min(chr_size)
        })
    })
    .assume_sorted()