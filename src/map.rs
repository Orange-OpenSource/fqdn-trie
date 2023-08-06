use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use crate::{HasFqdn, FQDN, Fqdn};
use crate::trie::InnerTrie;

impl<T> HasFqdn for (FQDN,T) {
    fn fqdn(&self) -> &Fqdn { &self.0 }
}

pub struct FqdnTrieMap<T> {
    inner: InnerTrie<(FQDN,T)>
}

impl<T> FqdnTrieMap<T> {

    #[inline]
    pub fn new(root: T) -> Self
    {
        Self { inner: InnerTrie::new((FQDN::default(), root)) }
    }

    #[inline]
    pub fn with_capacity(root: T, capacity: usize) -> Self
    {
        Self { inner: InnerTrie::with_capacity((FQDN::default(), root), capacity) }
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
    pub fn get<K:AsRef<Fqdn>>(&self, look: K) -> Option<&T>
    {
        self.inner.get_exact_leaf(look.as_ref()).map(|(_,l)| l)
    }

    #[inline]
    pub fn get_key_value<K:AsRef<Fqdn>>(&self, look: K) -> Option<(&Fqdn,&T)>
    {
        self.inner.get_exact_leaf(look.as_ref())
            .map(|(f,v)| (f.as_ref(), v))
    }

    #[inline]
    pub fn get_mut<K:AsRef<Fqdn>>(&mut self, look: K) -> Option<&mut T>
    {
        self.inner.get_exact_leaf_mut(look.as_ref()).map(|(_,l)| l)
    }

    #[inline]
    pub fn lookup<K:AsRef<Fqdn>>(&self, look: K) -> &T
    {
        &self.inner.lookup(look.as_ref()).1
    }

    #[inline]
    pub fn lookup_key_value<K:AsRef<Fqdn>>(&self, look: K) -> (&Fqdn,&T)
    {
        let (f,v) = self.inner.lookup(look.as_ref());
        (f.as_ref(), v)
    }

    #[inline]
    pub fn lookup_mut<K:AsRef<Fqdn>>(&mut self, look: K) -> &mut T
    {
        &mut self.inner.lookup_mut(look.as_ref()).1
    }

    #[inline]
    pub fn insert(&mut self, fqdn:FQDN, added: T) -> bool
    {
        self.inner.insert((fqdn,added))
    }

    #[inline]
    pub fn remove<K:AsRef<Fqdn>>(&mut self, removed: K) -> Option<(FQDN,T)>
    {
        self.inner.remove(removed.as_ref())
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
        let mut triemap = Self::new(T::default());
        triemap.extend(iter);
        triemap
    }
}


impl<I:AsRef<Fqdn>,T:AsRef<Fqdn>> Index<I> for FqdnTrieMap<T>
{
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.lookup(index)
    }
}

impl<I:AsRef<Fqdn>,T:AsRef<Fqdn>> IndexMut<I> for FqdnTrieMap<T>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.lookup_mut(index)
    }
}