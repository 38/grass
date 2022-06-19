from pygrass import RustEnv

def genomecov(input):
    """ Although the functionality needs to be implemented with inline Rust code mostly,
        But GRASS make this code fragement reusable for any intervals. 
        For example, you can also call 
            
            genomecov(input.merge_overlaps())

    """
    RustEnv(input = input)\
        .inline_rust("""
            use grass_runtime::{
                algorithm::Components,
                Genome,
                ChrRef
            };
            use std::collections::BTreeMap;
            let mut genome_wide = BTreeMap::<usize, usize>::new();
            let mut chr_wide = BTreeMap::<usize, usize>::new();
            let mut current_chr = Genome::get_chr_by_id(0).unwrap();
            let mut current_pos = 0;
            let mut current_dep = 0;
            fn print_histogram(chr: Option<&ChrRef<'static>>, stat: &BTreeMap<usize, usize>) {
                let chr_size = if let Some(chr) = chr { 
                    chr.get_chr_size().unwrap() 
                } else {
                    Genome::get_chrom_sizes().into_iter().map(|(_, s)| s).sum::<usize>()
                };
                let chr_name = if let Some(chr) = chr {
                    chr.get_chr_name()
                } else {
                    "genome"
                };
                let counted_bases = chr_size - stat.values().sum::<usize>();
                println!("{}\t0\t{}\t{}\t{}", chr_name, counted_bases, chr_size, counted_bases as f64 / chr_size as f64);
                for (k, v) in stat {
                    println!("{}\t{}\t{}\t{}\t{}", chr_name, k, v, chr_size, *v as f64 / chr_size as f64);
                }
            }
            for comp in input.components() {
                let (chr, pos) = comp.position();
                let depth = comp.depth();
                while current_chr != chr {
                    print_histogram(Some(&current_chr), &chr_wide);
                    current_pos = 0;
                    current_chr = Genome::get_chr_by_id(current_chr.id().unwrap() + 1).unwrap();
                    chr_wide.clear();
                }
                if current_dep > 0 {
                    *chr_wide.entry(current_dep).or_default() += (pos - current_pos) as usize;
                    *genome_wide.entry(current_dep).or_default() += (pos - current_pos) as usize;
                }
                current_dep = depth;
                current_pos = pos;
            }
            print_histogram(None, &genome_wide);

        """)

