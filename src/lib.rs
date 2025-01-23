//! This crate provides two data structures based on FQDN tries in order
//! to provide very fast lookup in the FQDN hierarchy.
//!
//! The trie implementation is optimized to FQDN context and follows these rules:
//! * the search algorithm finds the longuest domain suffix
//! * the algorithm is case-insensitive
//! * the internal structure exploits the range of allowed characters in FQDN

mod trie;
mod set;
mod map;

use fqdn::{FQDN,Fqdn};
pub use set::FqdnTrieSet;
pub use map::FqdnTrieMap;

/// Associate a FQDN to a structure.
trait HasFqdn {
    /// Get the FQDN which is associated to this trait.
    fn fqdn(&self) -> &Fqdn;
}

// size of the alphabet
pub(crate) const ALPHABET_SIZE: usize = 40; // we should also count the '_' and '#'

// in order to decrease the necessary memory, this table reduces the search space only
// to allowed chars in FQDN, i.e. a-z, A-Z, 0-9, -, _ and #.
// all the others are treated equally (i.e. as a dot)
// this is case insensitive (lower and upper case give the same index)

pub(crate) const ALPHABET: [u8;256] = [
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,   //  16
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,   //  32
    0, 0, 0,39, 0, 0, 0, 0,      0, 0, 0, 0, 0,37, 0, 0,   //  48 (# et -)
    27,28,29,30,31,32,33,34,    35,36, 0, 0, 0, 0, 0, 0,   //  64 (0-9)
    0, 1, 2, 3, 4, 5, 6, 7,      8, 9,10,11,12,13,14,15,   //  80 (A-O)
    16,17,18,19,20,21,22,23,    24,25,26, 0, 0, 0, 0,38,   //  96 (P-Z et _)
    0, 1, 2, 3, 4, 5, 6, 7,      8, 9,10,11,12,13,14,15,   // 112 (a-o)
    16,17,18,19,20,21,22,23,    24,25,26, 0, 0, 0, 0, 0,   // 128 (p-z)
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,      0, 0, 0, 0, 0, 0, 0, 0
];