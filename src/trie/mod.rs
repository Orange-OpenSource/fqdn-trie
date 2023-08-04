mod lookup;
mod remove;
mod branching;
mod index;
mod insert;

mod graphviz;
mod check;

use fqdn::*;

use index::*;
use crate::trie::branching::Branching;
use std::ops::{Index, IndexMut};
use crate::{HasFqdn, ALPHABET_SIZE};


#[derive(Debug,Clone)]
pub(crate) struct InnerTrie<T:HasFqdn> {
    branching: Vec<Branching>,
    leaf: Vec<T>
}


impl<T:HasFqdn+Default> Default for InnerTrie<T>
{
    fn default() -> Self {
        Self::new(T::default())
    }
}


impl<T:HasFqdn> InnerTrie<T> {

    pub(crate) fn new(root: T) -> Self
    {
        assert!( root.fqdn().is_root(), "the root data should be associated with the root (empty) FQDN" );
        Self {
            branching: vec![Branching::root()],
            leaf: vec![root],
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize
    {
        self.leaf.len()
    }

    pub(crate) fn with_capacity(root: T, capacity: usize) -> Self
    {
        assert!( root.fqdn().is_root(), "the root data should be associated with the root (empty) FQDN" );
        assert!( capacity <= MAX_INDEX, "exceeded capacity") ;

        let mut branching = Vec::with_capacity(capacity/2);
        branching.push(Branching::root());

        let mut leaf = Vec::with_capacity(capacity);
        leaf.push(root);

        Self { branching, leaf }
    }

    pub(crate) fn shrink_to_fit(&mut self)
    {
        self.branching.shrink_to_fit();
        self.leaf.shrink_to_fit();
    }

    pub(crate) fn reserve(&mut self, additional: usize)
    {
        self.leaf.reserve(additional);
        if self.leaf.capacity()/2 > self.branching.capacity() {
            self.branching.reserve(self.leaf.capacity() / 2 - self.branching.capacity());
        }
    }

    fn get_next_node(&self, b: BranchingIndex, f:&Fqdn) -> NodeIndex
    {
        if self[b].pos <= f.as_bytes().len() {
            self[b].child_from_letter(f) // std behavior
        } else if self[b].pos == (f.as_bytes().len() - 1) {
            self[b].child_from_dot() // just of first label length
        } else {
            self[b].escape.into() // too short FQDN
        }
    }

    /// Replaces recursively the index of escape in the trie (starting at the specified branching)
    pub(crate) fn replace_escape_leaf(&mut self, b: BranchingIndex, to_replace: LeafIndex, replacement: LeafIndex)
    {
        if self[b].escape == to_replace {
            self[b].escape = replacement;
            for i in 0..ALPHABET_SIZE {
                let c = self[b].child_from_index(i);
                if c.is_branching() {
                    // update recursively
                    self.replace_escape_leaf(c.into(), to_replace, replacement);
                } else if c == to_replace {
                    // effective location to replace
                    *self[b].child_from_index_mut(i) = replacement.into();
                }
            }
        }
    }

    /// Search one matching leaf (among all possible) that match the specified branching.
    /// There is always a result since the leaf associated to the root always matches.
    pub(crate) fn find_one_matching_leaf(&self, mut b: BranchingIndex) -> LeafIndex
    {
        loop {
            let escape = self[b].escape;
            if self[self[b].parent].escape != escape {
                return escape;
            }
            match self[b].child.iter()
                .filter(|&&i| i != escape)
                .next()
            {
                None => return LeafIndex::default(), // empty trie...
                Some(&i) if i.is_leaf() => return i.into(),
                Some(&i) if i.is_root() => return LeafIndex::default(), // ??? useful TODO: check
                Some(&i) => b = i.into() // continue the loop (descending into the trie)
            }
        }
    }
}


impl<T:HasFqdn> Index<BranchingIndex> for InnerTrie<T>
{
    type Output = Branching;

    fn index(&self, i: BranchingIndex) -> &Self::Output
    {
        debug_assert!( i.index() < self.branching.len());
        unsafe { self.branching.get_unchecked(i.index())}
    }
}

impl<T:HasFqdn> IndexMut<BranchingIndex> for InnerTrie<T>
{
    fn index_mut(&mut self, i: BranchingIndex) -> &mut Self::Output
    {
        debug_assert!( i.index() < self.branching.len());
        unsafe { self.branching.get_unchecked_mut(i.index())}
    }
}

impl<T:HasFqdn> Index<LeafIndex> for InnerTrie<T>
{
    type Output = T;

    fn index(&self, i: LeafIndex) -> &Self::Output
    {
        debug_assert!( i.index() < self.leaf.len());
        unsafe { self.leaf.get_unchecked(i.index()) }
    }
}

impl<T:HasFqdn> IndexMut<LeafIndex> for InnerTrie<T>
{
    fn index_mut(&mut self, i: LeafIndex) -> &mut Self::Output
    {
        debug_assert!( i.index() < self.leaf.len());
        unsafe { self.leaf.get_unchecked_mut(i.index())}
    }
}
