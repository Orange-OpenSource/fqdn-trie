use std::ops::{Add, Sub, AddAssign, SubAssign};
use crate::{ALPHABET, Fqdn};

pub(crate) const MAX_INDEX:usize = i32::MAX as usize;


/// position of relevant byte in FQDN
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct ByteIndex(u32);

impl Default for ByteIndex {
    /// the reversed position of last relevant character
    /// (skipping the nul trailing byte, so it starts at 2...)
    fn default() -> Self { Self(2) }
}

impl From<usize> for ByteIndex {
    fn from(i: usize) -> Self {
        debug_assert!( i <= MAX_INDEX );
        debug_assert!( i >= 2, "should be >= 2 since should skip the trailing nul" );
        Self(i as u32)
    }
}


impl ByteIndex
{
    #[inline]
    fn check(v: u32) -> u32
    {
        debug_assert!( v as usize <= MAX_INDEX );
        debug_assert!( v >= 2, "should be >= 2 since should skip the trailing nul" );
        v
    }

    #[inline]
    pub(super) fn get(&self, fqdn: &Fqdn) -> usize
    {
        let bytes = fqdn.as_bytes();
        debug_assert!( self <= &bytes.len());

        unsafe {
            // we are safe here because we control the index before
            // TODO: using of unchecked_sub here to avoid useless overflow checking
            let byte = *bytes.get_unchecked(bytes.len() - self.0 as usize);

            // we are safe here since ALPHABET cover all the 256 possible value.
            *ALPHABET.get_unchecked(byte as usize) as usize
        }
    }

    #[inline] pub(super) fn index(&self) -> usize { self.0 as usize }
}

impl AddAssign<u32> for ByteIndex { fn add_assign(&mut self, rhs: u32) { self.0 = ByteIndex::check(self.0+rhs); } }
impl SubAssign<u32> for ByteIndex { fn sub_assign(&mut self, rhs: u32) { self.0 -= ByteIndex::check(self.0-rhs); } }

impl Add for ByteIndex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self(ByteIndex::check(self.0 + rhs.0)) }
}

impl Sub for ByteIndex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self(ByteIndex::check(self.0 - rhs.0)) }
}

impl PartialEq<usize> for ByteIndex {
    fn eq(&self, other: &usize) -> bool {
        (self.0 as usize).eq(other)
    }
}

impl PartialOrd<usize> for ByteIndex {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        (self.0 as usize).partial_cmp(other)
    }
}

/// index of any node (leaf or branching, work as union)
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct NodeIndex(pub(crate) i32);

/// index of a leaf of the trie
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct LeafIndex(i32);

/// index of a branching of the trie
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct BranchingIndex(i32);

impl Default for LeafIndex {
    /// leaf associated to the root of the trie
    #[inline] fn default() -> Self { Self(!0) }
}

impl Default for BranchingIndex {
    /// branching root
    #[inline] fn default() -> Self { Self(0) }
}


impl NodeIndex {
    #[inline] pub(crate) fn is_root(&self) -> bool { self.0 == 0 }
    #[inline] pub(crate) fn is_branching(&self) -> bool { self.0 >= 0 }
    #[inline] pub(crate) fn is_leaf(&self) -> bool { self.0 < 0 }
}

impl LeafIndex {
    #[inline] pub(crate) fn is_root_domain(&self) -> bool { !self.0 == 0 }
    #[inline] pub(crate) fn index(&self) -> usize { !self.0 as usize }
}

impl BranchingIndex {
    #[inline] pub(crate) fn is_root(&self) -> bool { self.0 == 0 }
    #[inline] pub(crate) fn index(&self) -> usize { self.0 as usize }
}

impl From<NodeIndex> for LeafIndex
{
    #[inline]
    fn from(i: NodeIndex) -> Self {
        debug_assert!( i.is_leaf() );
        Self(i.0)
    }
}

impl From<usize> for LeafIndex
{
    #[inline]
    fn from(i: usize) -> Self {
        debug_assert!( i <= MAX_INDEX );
        Self(!(i as i32))
    }
}

impl From<usize> for BranchingIndex
{
    #[inline]
    fn from(i: usize) -> Self {
        debug_assert!( i <= MAX_INDEX );
        Self(i as i32)
    }
}

impl From<NodeIndex> for BranchingIndex
{
    #[inline]
    fn from(i: NodeIndex) -> Self {
        debug_assert!( i.is_branching() );
        Self(i.0)
    }
}


impl From<LeafIndex> for NodeIndex {
    #[inline] fn from(i: LeafIndex) -> Self { Self(i.0) }
}

impl From<BranchingIndex> for NodeIndex {
    #[inline] fn from(i: BranchingIndex) -> Self { Self(i.0) }
}

impl PartialEq<LeafIndex> for NodeIndex {
    #[inline] fn eq(&self, other: &LeafIndex) -> bool { self.0 == other.0 }
}

impl PartialEq<BranchingIndex> for NodeIndex {
    #[inline] fn eq(&self, other: &BranchingIndex) -> bool { self.0 == other.0 }
}

impl PartialEq<NodeIndex> for BranchingIndex {
    #[inline] fn eq(&self, other: &NodeIndex) -> bool { self.0 == other.0 }
}

impl PartialEq<NodeIndex> for LeafIndex {
    #[inline] fn eq(&self, other: &NodeIndex) -> bool { self.0 == other.0 }
}

use std::fmt;
use std::cmp::Ordering;

impl fmt::Debug for ByteIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl fmt::Debug for LeafIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl fmt::Debug for BranchingIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl fmt::Debug for NodeIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}