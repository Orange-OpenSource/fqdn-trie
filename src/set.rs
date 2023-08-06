use std::iter::FromIterator;
use std::ops::Index;

use crate::{HasFqdn, Fqdn};
use crate::trie::InnerTrie;


pub struct FqdnTrieSet<T:AsRef<Fqdn>> {
    inner: InnerTrie<TrieSetElt<T>>
}

struct TrieSetElt<T:AsRef<Fqdn>>(T);
impl<T:AsRef<Fqdn>> HasFqdn for TrieSetElt<T> {
    #[inline] fn fqdn(&self) -> &Fqdn { self.0.as_ref() }
}

impl<T:AsRef<Fqdn>> FqdnTrieSet<T> {

    #[inline]
    pub fn new(root: T) -> Self
    {
        Self { inner: InnerTrie::new(TrieSetElt(root)) }
    }

    #[inline]
    pub fn with_capacity(root: T, capacity: usize) -> Self
    {
        Self { inner: InnerTrie::with_capacity(TrieSetElt(root), capacity) }
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize)
    {
        self.inner.reserve(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self)
    {
        self.inner.shrink_to_fit()
    }

    #[inline]
    pub fn get<K: AsRef<Fqdn>>(&self, look: K) -> Option<&T>
    {
        self.inner.get_exact_leaf(look.as_ref()).map(|x| &x.0)
    }

    #[inline]
    pub fn lookup<K:AsRef<Fqdn>>(&self, look: K) -> &T
    {
        &self.inner.lookup(look.as_ref()).0
    }

    #[inline]
    pub fn insert(&mut self, added: T) -> bool
    {
        self.inner.insert(TrieSetElt(added))
    }

    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T>
    {
        self.inner.replace(TrieSetElt(value)).map(|x| x.0)
    }

    #[inline]
    pub fn remove<K:AsRef<Fqdn>>(&mut self, removed: K) -> Option<T>
    {
        self.inner.remove(removed.as_ref()).map(|x| x.0)
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

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }
}

impl<T:AsRef<Fqdn>> Extend<T> for FqdnTrieSet<T>
{
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I)
    {
        iter.into_iter()
            .for_each(| item | { self.replace(item); } )
    }
}

impl<T:AsRef<Fqdn>+Default> FromIterator<T> for FqdnTrieSet<T>
{
    fn from_iter<I:IntoIterator<Item=T>>(iter: I) -> Self
    {
        let mut trieset = Self::new(T::default());
        trieset.extend(iter);
        trieset
    }
}

impl<I:AsRef<Fqdn>,T:AsRef<Fqdn>> Index<I> for FqdnTrieSet<T>
{
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.lookup(index)
    }
}
