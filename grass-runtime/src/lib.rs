mod genome;
mod ioutils;
mod file;

pub mod property;
pub mod record;
pub mod algorithm;

pub use genome::{Genome, ChrRef};
pub use file::LineRecordStreamExt;

pub mod prelude {
}


#[cfg(test)]
mod test {
    use crate::Genome;

    #[test]
    fn test_genome_storage() {
        let id = Genome::query_chr("chr1").get_id_or_update();
        assert_eq!(id, 0);
        assert_eq!(Genome::query_chr("chr1").id(), Some(0));
        assert_eq!(Genome::query_chr("chr1").verify_size_or_update(100), true);
        assert_ne!(Genome::query_chr("chr1").verify_size_or_update(200), true);
        assert_eq!(Genome::query_chr("chr1").get_chr_size(), Some(100));
        assert_eq!(Genome::query_chr("chr1").to_string(), "chr1");
    }
}
