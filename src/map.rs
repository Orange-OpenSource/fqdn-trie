use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use crate::{HasFqdn, FQDN, Fqdn};
use crate::trie::InnerTrie;

impl<K:AsRef<Fqdn>,T> HasFqdn for (K,T) {
    fn fqdn(&self) -> &Fqdn { self.0.as_ref() }
}

pub struct FqdnTrieMap<K:AsRef<Fqdn>,T> {
    inner: InnerTrie<(K,T)>
}

impl<K:AsRef<Fqdn>,T> FqdnTrieMap<K,T> {

    #[inline]
    pub fn new(root: T) -> Self
        where K:Default
    {
        let fqdn = K::default();
        assert_eq!( *fqdn.as_ref(), FQDN::default());
        Self { inner: InnerTrie::new((fqdn, root)) }
    }

    #[inline]
    pub fn with_key_root(key: K, root: T) -> Self
    {
        assert_eq!( *key.as_ref(), FQDN::default());
        Self { inner: InnerTrie::new((key, root)) }
    }

    #[inline]
    pub fn with_capacity(root: T, capacity: usize) -> Self
        where K:Default
    {
        let fqdn = K::default();
        assert_eq!( *fqdn.as_ref(), FQDN::default());
        Self { inner: InnerTrie::with_capacity((fqdn, root), capacity) }
    }

    #[inline]
    pub fn with_key_root_and_capacity(key: K, root: T, capacity: usize) -> Self
        where K:Default
    {
        assert_eq!( *key.as_ref(), FQDN::default());
        Self { inner: InnerTrie::with_capacity((key, root), capacity) }
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
    pub fn get<Q:AsRef<Fqdn>>(&self, look: Q) -> Option<&T>
    {
        self.inner.get_exact_leaf(look.as_ref()).map(|(_,l)| l)
    }

    #[inline]
    pub fn get_key_value<Q:AsRef<Fqdn>>(&self, look: Q) -> Option<(&K,&T)>
    {
        self.inner.get_exact_leaf(look.as_ref()).map(|(f,v)| (f, v))
    }

    #[inline]
    pub fn get_mut<Q:AsRef<Fqdn>>(&mut self, look: Q) -> Option<&mut T>
    {
        self.inner.get_exact_leaf_mut(look.as_ref()).map(|(_,l)| l)
    }

    #[inline]
    pub fn lookup<Q:AsRef<Fqdn>>(&self, look: Q) -> &T
    {
        &self.inner.lookup(look.as_ref()).1
    }

    #[inline]
    pub fn lookup_key_value<Q:AsRef<Fqdn>>(&self, look: Q) -> (&K,&T)
    {
        let (f,v) = self.inner.lookup(look.as_ref());
        (f, v)
    }

    #[inline]
    pub fn lookup_mut<Q:AsRef<Fqdn>>(&mut self, look: Q) -> &mut T
    {
        &mut self.inner.lookup_mut(look.as_ref()).1
    }

    #[inline]
    pub fn insert(&mut self, fqdn:K, added: T) -> bool
    {
        self.inner.insert((fqdn,added))
    }

    #[inline]
    pub fn remove<Q:AsRef<Fqdn>>(&mut self, removed: Q) -> Option<(K,T)>
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

impl<K:AsRef<Fqdn>,T> Extend<(K,T)> for FqdnTrieMap<K,T>
{
    fn extend<I: IntoIterator<Item=(K,T)>>(&mut self, iter: I)
    {
        iter.into_iter().for_each(| (key,val) | { self.insert(key, val); } )
    }
}


impl<K:AsRef<Fqdn>+Default,T:Default> FromIterator<(K,T)> for FqdnTrieMap<K,T>
{
    fn from_iter<I:IntoIterator<Item=(K,T)>>(iter: I) -> Self
    {
        let mut triemap = Self::new(T::default());
        triemap.extend(iter);
        triemap
    }
}


impl<I:AsRef<Fqdn>,K:AsRef<Fqdn>,T> Index<I> for FqdnTrieMap<K,T>
{
    type Output = T;
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.lookup(index)
    }
}

impl<I:AsRef<Fqdn>,K:AsRef<Fqdn>,T> IndexMut<I> for FqdnTrieMap<K,T>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.lookup_mut(index)
    }
}