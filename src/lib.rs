//! # Compute, compare and search signatures for nucleotide (DNA/RNA) and protein sequences.
//!
//! sourmash is a command-line tool and Python library for computing
//! [MinHash sketches][0] from DNA sequences, comparing them to each other,
//! and plotting the results.
//! This allows you to estimate sequence similarity between even very
//! large data sets quickly and accurately.
//!
//! [0]: https://en.wikipedia.org/wiki/MinHash
//!
//! sourmash can be used to quickly search large databases of genomes
//! for matches to query genomes and metagenomes.
//!
//! sourmash also includes k-mer based taxonomic exploration and
//! classification routines for genome and metagenome analysis. These
//! routines can use the NCBI taxonomy but do not depend on it in any way.
//! Documentation and further examples for each module can be found in the module descriptions below.

pub mod errors;

#[macro_use]
pub mod utils;

pub mod index;

pub mod signature;
pub mod sketch;

#[cfg(feature = "from-finch")]
pub mod from;

use cfg_if::cfg_if;
use murmurhash3::murmurhash3_x64_128;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub mod wasm;
    } else {
        pub mod ffi;

        pub mod cmd;
    }
}

type HashIntoType = u64;

pub fn _hash_murmur(kmer: &[u8], seed: u64) -> u64 {
    murmurhash3_x64_128(kmer, seed).0
}