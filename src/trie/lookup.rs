use crate::trie::InnerTrie;
use crate::trie::index::{BranchingIndex, LeafIndex, NodeIndex};
use crate::Fqdn;
use crate::HasFqdn;

impl<T: HasFqdn> InnerTrie<T> {

    pub(crate) fn lookup(&self, fqdn:&Fqdn) -> &T
    {
        let (_,l) = self.inner_lookup(fqdn);
        unsafe { self.leaf.get_unchecked(l.index()) }
    }

    pub(crate) fn lookup_mut(&mut self, fqdn:&Fqdn) -> &mut T
    {
        let (_,l) = self.inner_lookup(fqdn);
        unsafe { self.leaf.get_unchecked_mut(l.index()) }
    }

    pub(super) fn inner_lookup(&self, f: &Fqdn) -> (BranchingIndex, LeafIndex)
    {
        let mut n: NodeIndex = BranchingIndex::default().into(); // the desired leaf index
        let mut b : BranchingIndex; // the branch index (parent of l)

        loop {
            b = n.into(); // we kept the last visited branching
            n = self.get_next_node(b, f);
            if n.is_leaf() { break }
        }
        let mut l : LeafIndex = n.into();
        if l != self[b].escape {
            // if the leaf is a subdomain, it’s done !
            // if not, we will check the escape nodes...
            if f.is_subdomain_of( self[l].fqdn() ) {
                return (b, l); // found !!
            } else {
                l = self[b].escape;
            }
        }
        //  we need to find the first matching escape
        while !f.is_subdomain_of(self[l].fqdn()) {
            loop {
                b = self[b].parent;
                if l != self[b].escape { break }
            }
            l = self[b].escape;
        }
        (b, l)
    }

    pub(crate) fn get_exact_leaf(&self, fqdn:&Fqdn) -> Option<&T>
    {
        self.inner_get_exact_leaf(fqdn).map(|l| &self[l])
    }

    pub(crate) fn get_exact_leaf_mut(&mut self, fqdn:&Fqdn) -> Option<&mut T>
    {
        if let Some(l) = self.inner_get_exact_leaf(fqdn) {
            Some(&mut self[l])
        } else {
            None
        }
    }

    fn inner_get_exact_leaf(&self, f: &Fqdn) -> Option<LeafIndex>
    {
        // todo: improve that (lookup could be shorten)
        let (_,l) = self.inner_lookup(f);
        self[l].fqdn().eq(f).then_some(l)
    }
}