use crate::trie::InnerTrie;
use crate::HasFqdn;
use crate::trie::index::{LeafIndex, ByteIndex, BranchingIndex, NodeIndex, MAX_INDEX};
use crate::Fqdn;
use crate::trie::branching::Branching;
use std::mem;

impl<T: HasFqdn> InnerTrie<T> {

    /// Inserts a new branching in the trie and returns its index.
    fn insert_suffix_branching(&mut self, b: BranchingIndex, e: LeafIndex, x: NodeIndex, p: ByteIndex, f: &Fqdn) -> BranchingIndex
    {
        debug_assert!( self[b].pos < p);
        debug_assert!( self.branching.len() < MAX_INDEX );

        let n = self.branching.len().into(); // the new branching
        let mut nb = Branching::new(b, e, p);

        *nb.child_from_letter_mut(f) = x;
        if x.is_branching() {
            let x : BranchingIndex = x.into();
            debug_assert!(self[x].pos > p);
            self[x].parent = n;
            if self[x].escape == self[b].escape {
                self.replace_escape_leaf(x, self[b].escape, e);
            }
        }
        *self[b].child_from_letter_mut(f) = n.into();
        self.branching.push(nb);
        n
    }


    /// Search a concurrent leaf of the specified FQDN
    /// There is always a result since the leaf associated to the root always matches.
    fn find_one_concurrent_leaf(&self, f:&Fqdn) -> LeafIndex
    {
        let mut n: NodeIndex = BranchingIndex::default().into();
        let mut b;

        loop {
            b = n.into();
            n = self.get_next_node(b, f);

            if n.is_leaf() {
                let l = n.into();
                if l != self[b].escape {
                    return l;
                } else {
                    return self.find_one_matching_leaf(b)
                }
            }
        }
    }

    pub fn replace(&mut self, mut value: T) -> Option<T>
    {
        debug_assert!(self.check_consistency());

        let added_fqdn = value.fqdn();

        // search the insertion point
        let (b,l) = self.inner_lookup(added_fqdn);

        if added_fqdn.eq(self[l].fqdn()) {
            // okc this fqdn was already present
            std::mem::swap(&mut value, &mut self[l]);
            Some(value)
        } else {
            self.real_insert(value, b, l);
            None
        }
    }

    pub fn insert(&mut self, added: T) -> bool
    {
        debug_assert!(self.check_consistency());

        // search the insertion point
        let (b, l) = self.inner_lookup(added.fqdn());

        if added.fqdn().eq(self[l].fqdn()) {
            false
        } else {
            self.real_insert(added, b, l);
            true
        }
    }

    fn real_insert(&mut self, added: T, mut b: BranchingIndex, l: LeafIndex)
    {
        let added_fqdn = added.fqdn();

        let added_leaf: LeafIndex = self.leaf.len().into();

        if self[b].escape != l {
            // the new inserted FQDN extends an already existing FQDN at a non-escape position
            let position = self[l].fqdn().as_bytes().len() + 1;
            self.insert_suffix_branching(b,l,added_leaf.into(), position.into(), added_fqdn);

        } else {
            // todo: change the structure to avoid this take
            let mut leaves = mem::take(&mut self.leaf);

            let derived = unsafe {
                leaves.get_unchecked(self.find_one_concurrent_leaf(added_fqdn).index()).fqdn()
            };
            if derived.is_root() {
                // trie is empty, this is the first insertion
                *self[b].child_from_letter_mut(added_fqdn) = added_leaf.into();
            } else if derived.is_subdomain_of(added_fqdn) {
                // the added fqdn will be in an escape position
                let p : ByteIndex = (added_fqdn.as_bytes().len() + 1).into(); // discriminant position
                // go up to the insertion point
                while p < self[b].pos {
                    debug_assert_ne!(b, self[b].parent);
                    b = self[b].parent
                }
                if p == self[b].pos {
                    self.replace_escape_leaf(b, self[b].escape, added_leaf);
                } else {
                    let x = self[b].child_from_letter(derived);
                    self.insert_suffix_branching(b, added_leaf, x, p, derived);
                }
            } else {
                // case of a new concurrent FQDN
                let mut p : ByteIndex = (leaves[l.index()].fqdn().as_bytes().len() + 1).into();
                if p <= derived.as_bytes().len() {
                    while p.get(added_fqdn) == p.get(derived) {  p += 1; }
                }
                debug_assert!( p <= added_fqdn.as_bytes().len()+1); // we already have a discriminant position

                // go up to the insertion point
                while p < self[b].pos {
                    b = self[b].parent
                }
                let branching = &mut self[b];
                let discriminant = branching.pos.get(added_fqdn);
                if p == branching.pos
                    && branching.child_from_index(discriminant) == branching.escape {
                    *branching.child_from_index_mut(discriminant) = added_leaf.into();
                } else {
                    // a new branching should be inserted
                    let x = branching.child_from_index(discriminant);
                    let nb = self.insert_suffix_branching(b,l, x, p, derived);
                    *self[nb].child_from_letter_mut(added_fqdn) = added_leaf.into();
                }
            }
            mem::swap(&mut self.leaf, &mut leaves);
        }
        self.leaf.push(added);
    }


}