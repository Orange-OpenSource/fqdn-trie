use crate::ALPHABET_SIZE;
use crate::Fqdn;
use crate::trie::*;

// NOTE (future plans): this branching needs lot of memory space (about 39 children, one per letter)
// In future release, letter will be associated to a primary and a secondary index
// in order to reduce size with small impact on lookup
//
// Children could be reduced to 7 since we’ll have 7*7 nodes with two levels
// If fact, we probably isolate some letters (such 0, but perhaps more ?) immediately at the primary level
#[derive(Debug,Copy,Clone)]
pub(crate) struct Branching {
    pub(crate) pos: ByteIndex, // position of the char to check at this node (always > 1 to skip nul ending byte)
    pub(crate) parent: BranchingIndex, // first available child (for updating)
    pub(crate) child: [NodeIndex; ALPHABET_SIZE],
    pub(crate) escape: LeafIndex, // if fqdn is too short
}

impl Branching {

    pub(crate) fn root() -> Self {
        Self::new(BranchingIndex::default(),LeafIndex::default(),ByteIndex::default())
    }

    pub(crate) fn new(parent: BranchingIndex, escape: LeafIndex, pos:ByteIndex) -> Self
    {
        Self {
            pos, escape, parent,
            child: [escape.into(); ALPHABET_SIZE],
        }
    }

    #[inline]
    pub(crate) fn child_from_dot(&self) -> NodeIndex {
        self.child_from_index(0)
    }

    #[inline]
    pub(crate) fn child_from_letter(&self, bytes: &Fqdn) -> NodeIndex {
        self.child_from_index(self.pos.get(bytes) as usize)
    }

    #[inline]
    pub(crate) fn child_from_letter_mut(&mut self, bytes: &Fqdn) -> &mut NodeIndex {
        self.child_from_index_mut(self.pos.get(bytes) as usize)
    }

    #[inline]
    pub(crate) fn child_from_index(&self, i:usize) -> NodeIndex {
        unsafe { *self.child.get_unchecked(i) }
    }

    #[inline]
    pub(crate) fn child_from_index_mut(&mut self, i:usize) -> &mut NodeIndex {
        unsafe { self.child.get_unchecked_mut(i) }
    }
}
