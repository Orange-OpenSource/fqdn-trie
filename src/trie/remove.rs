use crate::trie::InnerTrie;
use crate::{Fqdn, FQDN};
use crate::{HasFqdn, ALPHABET_SIZE};
use crate::trie::index::{BranchingIndex, LeafIndex};

impl<T: HasFqdn> InnerTrie<T> {

    pub fn remove(&mut self, f: &Fqdn) -> Option<T>
    {
        debug_assert!(self.check_consistency());

        let (mut b,l) = self.inner_lookup(f);
        if f != self[l].fqdn() || l.is_root_domain() {
            // this suffix does not match anything
            return None;
        }

        if l == self[b].escape {
            // the suffixe to remove is an escape leaf node so
            // we need to go up to replace this escape by a new one
            while self[self[b].parent].escape == l {
                b = self[b].parent;
            }
            self.replace_escape_leaf(b, l, self[self[b].parent].escape);
        } else {
            // this is a regular child of the trie...
            // we just redirect this child to the escape value of the branching
            *self[b].child_from_letter_mut(f) = self[b].escape.into();
        }

        // if the associated branching is not the root, it could become useless
        // depending on how many children remain (more exactly, if only one remains)
        if !b.is_root() {
            let mut child_iter = self[b].child.iter()
                .filter(|&c| c.ne(&self[b].escape));

            let only_child = if self[b].escape == self[self[b].parent].escape {
                // the escape is originated from parent so we are sure that this branching has, at least, one child !
                // (so unwrapping the first iteration always succeeds)
                *child_iter.next().unwrap()
            } else {
                self[b].escape.into() // this is the escape origin branching
            };
            // now, we check if there is no more child (or this branching should be kept)
            // (or if the branching is the root, of course, we should also keep it !)
            if child_iter.next().is_none() {

                // here, we know that only one child remain for this branching
                // so it should be removed from the trie
                let parent = self[b].parent;
                *self[parent].child_from_letter_mut(f) = only_child;
                if only_child.is_branching() {
                    let only_child: BranchingIndex = only_child.into();
                    self[only_child].parent = parent;
                }

                // the removed branching should be swapped with the last one
                let swap = (self.branching.len() - 1).into();
                if b == swap {
                    // too easy, we just need to pop the last node
                    self.branching.pop();
                } else {
                    // the swap should be propagated
                    let fswap: FQDN = self[self.find_one_matching_leaf(swap)].fqdn().into();
                    let parent = self[swap].parent;
                    *self[parent].child_from_letter_mut(&fswap) = b.into();
                    self.branching.swap_remove(b.index());
                    (0..ALPHABET_SIZE).for_each(|i| {
                        let c = self[b].child[i];
                        if c.is_branching() {
                            let c: BranchingIndex = c.into();
                            self[c].parent = b;
                        }
                    });
                }
            }
        }

        let lastleaf : LeafIndex = (self.leaf.len() - 1).into();
        if l == lastleaf {
            self.leaf.pop()
        } else {
            // ok, we propagate the swap into the relevant branch
            // todo: suppress the following creation of FQDN ?
            let flast:FQDN = self[lastleaf].fqdn().into();
            let b = BranchingIndex::default();
            if self[b].child_from_letter(&flast) == lastleaf {
                *self[b].child_from_letter_mut(&flast) = l.into();
            } else {
                let mut n = self[b].child_from_letter(&flast);
                while n.is_branching() && !n.is_root() {
                    let b : BranchingIndex = n.into();
                    if self[b].escape == lastleaf {
                        self.replace_escape_leaf(b, lastleaf, l);
                        break;
                    }
                    let discriminant = if self[b].pos.index() == flast.as_bytes().len() {
                        0
                    } else {
                        self[b].pos.get(&flast)
                    };
                    if self[b].child_from_index(discriminant) == lastleaf {
                        *self[b].child_from_index_mut(discriminant) = l.into();
                        break;
                    }
                    n = self[b].child_from_index(discriminant);
                }
            }
            Some(self.leaf.swap_remove(l.index()))
        }
    }

}