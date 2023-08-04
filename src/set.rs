use std::iter::FromIterator;

use crate::{HasFqdn, Fqdn};
use crate::trie::InnerTrie;


pub struct FqdnTrieSet<T:HasFqdn> {
    inner: InnerTrie<T>
}

impl<T:HasFqdn> FqdnTrieSet<T> {

    pub fn new(root: T) -> Self
    {
        Self { inner: InnerTrie::new(root) }
    }

    pub fn with_capacity(root: T, capacity: usize) -> Self
    {
        Self { inner: InnerTrie::with_capacity(root, capacity) }
    }

    pub fn reserve(&mut self, additional: usize)
    {
        self.inner.reserve(additional)
    }

    pub fn shrink_to_fit(&mut self)
    {
        self.inner.shrink_to_fit()
    }

    pub fn lookup(&self, look: &Fqdn) -> &T
    {
        self.inner.lookup(look)
    }

    pub fn insert(&mut self, added: T) -> bool
    {
        self.inner.insert(added)
    }

    pub fn remove(&mut self, removed: &Fqdn) -> Option<T>
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

    pub fn len(&self) -> usize { self.inner.len() }
}

impl<T:HasFqdn> Extend<T> for FqdnTrieSet<T>
{
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I)
    {
        iter.into_iter()
            .for_each(| item | { self.insert(item); } )
    }
}

impl<T:HasFqdn+Default> FromIterator<T> for FqdnTrieSet<T>
{
    fn from_iter<I:IntoIterator<Item=T>>(iter: I) -> Self
    {
        iter.into_iter()
            .fold(Self::new(T::default()),
                  | mut trie, item | {
                      trie.insert(item);
                      trie
                  })
    }
}