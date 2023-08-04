use crate::HasFqdn;
use crate::trie::InnerTrie;
use crate::trie::index::BranchingIndex;

impl<T:HasFqdn> InnerTrie<T>
{
    pub(crate) fn check_consistency(&self) -> bool
    {
        self.check_parents() && self.check_leaf_fqdn()
    }

    // checks if any branching child knows its parent
    fn check_parents(&self) -> bool
    {
        self.branching.iter()
            .enumerate()
            .all(|(i,b)| {
                b.child.iter()
                    .filter_map(|&c|
                        if c.is_branching() { Some(BranchingIndex::from(c)) } else { None } )
                    .all(|c| self[c].parent.index() == i)
            })
    }

    // checks if any leaf are correctly set in the trie
    // (so we are able to find in through its fqdn)
    fn check_leaf_fqdn(&self) -> bool
    {
        self.leaf.iter()
            .enumerate()
            .all(|(i, b)|  self.inner_lookup(b.fqdn()).1.index() == i )
    }
}