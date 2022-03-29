mod file;
mod genome;
mod ioutils;

pub mod algorithm;
pub mod property;
pub mod record;

pub use file::LineRecordStreamExt;
pub use genome::{ChrRef, Genome};

pub mod prelude {}

#[cfg(test)]
mod test {
    use crate::Genome;

    #[test]
    fn test_genome_storage() {
        Genome::clear_genome_definition();
        let id = Genome::query_chr("chr1").get_id_or_update();
        assert_eq!(id, 0);
        assert_eq!(Genome::query_chr("chr1").id(), Some(0));
        assert_eq!(Genome::query_chr("chr1").verify_size_or_update(100), true);
        assert_ne!(Genome::query_chr("chr1").verify_size_or_update(200), true);
        assert_eq!(Genome::query_chr("chr1").get_chr_size(), Some(100));
        assert_eq!(Genome::query_chr("chr1").to_string(), "chr1");
    }
}
