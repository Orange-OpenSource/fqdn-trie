use std::iter::FromIterator;

use crate::{HasFqdn, FQDN, Fqdn};
use crate::trie::InnerTrie;

impl<T> HasFqdn for (FQDN,T) {
    fn fqdn(&self) -> &Fqdn { &self.0 }
}

pub struct FqdnTrieMap<T> {
    inner: InnerTrie<(FQDN,T)>
}

impl<T> FqdnTrieMap<T> {

    pub fn new(root: T) -> Self
    {
        Self { inner: InnerTrie::new((FQDN::default(), root)) }
    }

    pub fn with_capacity(root: T, capacity: usize) -> Self
    {
        Self { inner: InnerTrie::with_capacity((FQDN::default(), root), capacity) }
    }

    pub fn reserve(&mut self, additional: usize)
    {
        self.inner.reserve(additional)
    }

    pub fn shrink_to_fit(&mut self)
    {
        self.inner.shrink_to_fit()
    }

    pub fn get(&self, look: &Fqdn) -> Option<&T>
    {
        self.inner.get_exact_leaf(look).map(|(_,l)| l)
    }

    pub fn get_key_value(&self, look: &Fqdn) -> Option<&(FQDN,T)>
    {
        self.inner.get_exact_leaf(look)
    }

    pub fn get_mut(&mut self, look: &Fqdn) -> Option<&mut T>
    {
        self.inner.get_exact_leaf_mut(look).map(|(_,l)| l)
    }

    pub fn lookup(&self, look: &Fqdn) -> &T
    {
        &self.inner.lookup(look).1
    }

    pub fn lookup_key_value(&self, look: &Fqdn) -> &(FQDN,T)
    {
        &self.inner.lookup(look)
    }

    pub fn lookup_mut(&mut self, look: &Fqdn) -> &mut T
    {
        &mut self.inner.lookup_mut(look).1
    }

    pub fn insert(&mut self, fqdn:FQDN, added: T) -> bool
    {
        self.inner.insert((fqdn,added))
    }

    pub fn remove(&mut self, removed: &Fqdn) -> Option<(FQDN,T)>
    {
        self.inner.remove(removed)
    }

    #[cfg(feature= "graphviz")]
    pub fn generate_graphviz_file(&self, file: Option<&str>) -> std::io::Result<()> { self.inner.generate_graphviz_file(file) }

    #[cfg(feature= "graphviz")]
    pub fn generate_pdf_file(&self, file: Option<&str>) -> std::io::Result<()> { self.inner.generate_pdf_file(file) }

    #[cfg(target_os = "macos")]
    #[cfg(feature= "graphviz")]
    pub fn open_dot_view(&self) -> std::io::Result<()>
    {
        self.inner.open_dot_view()
    }
}

impl<T> Extend<(FQDN,T)> for FqdnTrieMap<T>
{
    fn extend<I: IntoIterator<Item=(FQDN, T)>>(&mut self, iter: I)
    {
        iter.into_iter()
            .for_each(| (key,val) | { self.insert(key, val); } )
    }
}


impl<T:Default> FromIterator<(FQDN,T)> for FqdnTrieMap<T>
{
    fn from_iter<I:IntoIterator<Item=(FQDN,T)>>(iter: I) -> Self
    {
        iter.into_iter()
            .fold(Self::new(T::default()),
                  | mut trie, (key,val) | {
                      trie.insert(key, val);
                      trie
                  })
    }
}