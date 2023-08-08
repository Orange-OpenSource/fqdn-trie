use std::iter::FromIterator;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut};

use crate::{HasFqdn, FQDN, Fqdn};
use crate::trie::InnerTrie;

impl<K:AsRef<Fqdn>,T> HasFqdn for (K,T) {
    fn fqdn(&self) -> &Fqdn { self.0.as_ref() }
}

/// A map of FQDN based on a suffix trie
pub struct FqdnTrieMap<K:AsRef<Fqdn>,T> {
    inner: InnerTrie<(K,T)>
}

impl<K:AsRef<Fqdn>,T> FqdnTrieMap<K,T> {

    /// Creates a new set with the root element.
    ///
    /// # Panics
    /// Panics if the FQDN of the root element is not the empty one (`.`)
    ///
    /// # Example
    /// ```
    /// use fqdn::FQDN;
    /// use fqdn_trie::FqdnTrieMap;
    /// let map = FqdnTrieMap::<FQDN,u32>::new(42);
    /// ```
    #[inline]
    pub fn new(root: T) -> Self
        where K:Default
    {
        let fqdn = K::default();
        assert_eq!( *fqdn.as_ref(), FQDN::default());
        Self { inner: InnerTrie::new((fqdn, root)) }
    }

    /// Creates a new set with the root element.
    ///
    /// # Panics
    /// Panics if the FQDN of the root element is not the empty one (`.`)
    ///
    /// # Example
    /// ```
    /// use fqdn::FQDN;
    /// use fqdn_trie::FqdnTrieMap;
    /// let map = FqdnTrieMap::with_key_root(FQDN::default(), 42);
    /// ```
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

    /// Reserves capacity for at least additional more elements to be inserted in the set.
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling reserve, capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    ///
    /// # Panics
    /// Panics if the new allocation size overflows usize.
    ///
    /// # Example
    /// ```
    /// use fqdn::FQDN;
    /// use fqdn_trie::FqdnTrieMap;
    /// let mut map = FqdnTrieMap::with_key_root(FQDN::default(), 42);
    /// map.reserve(100);
    /// ```
    #[inline]
    pub fn reserve(&mut self, additional: usize)
    {
        self.inner.reserve(additional)
    }

    /// Shrinks the capacity of the trie as much as possible.
    /// It will drop down as much as possible while maintaining the internal rules
    /// and possibly leaving some space in accordance with the resize policy.
    #[inline]
    pub fn shrink_to_fit(&mut self)
    {
        self.inner.shrink_to_fit()
    }

    /// Gets the element, if any, which exactly matches (i.e. equals) the given FQDN.
    ///
    /// It works like a "classical" hash map (but without computing the hash value).
    ///
    /// To find the longuest parent domain, consider [`Self::lookup`]
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieMap;
    ///
    /// let mut fqdns = FqdnTrieMap::with_key_root(FQDN::default(), 7);
    /// fqdns.insert(fqdn!("orange.com"), 42);
    /// fqdns.insert(fqdn!("mail.orange.com"), 87);
    ///
    /// assert_eq!( fqdns.get(fqdn!("orange.com")), Some(&42) );
    /// assert_eq!( fqdns.get(fqdn!("www.orange.com")), None);
    /// ```
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

    /// Gets the element which the longuest parent domain of the given FQDN.
    ///
    /// To use an exact match, consider [`Self::get`]
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieMap;
    ///
    /// let mut fqdns = FqdnTrieMap::with_key_root(FQDN::default(), 7);
    /// fqdns.insert(fqdn!("orange.com"), 42);
    /// fqdns.insert(fqdn!("mail.orange.com"), 87);
    ///
    /// assert_eq!( fqdns.lookup(fqdn!("orange.com")), &42 );
    /// assert_eq!( fqdns.lookup(fqdn!("www.orange.com")), &42);
    /// assert_eq!( fqdns.lookup(fqdn!("blue.com")), &7);
    /// ```
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
    pub fn insert(&mut self, fqdn:K, added: T) -> Option<T>
    {
        self.inner.replace((fqdn,added)).map(|(_,v)| v)
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    #[inline]
    pub fn remove<Q:AsRef<Fqdn>>(&mut self, removed: Q) -> Option<T>
    {
        self.inner.remove(removed.as_ref()).map(|(_,v)| v)
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in the map.
    #[inline]
    pub fn remove_entry<Q:AsRef<Fqdn>>(&mut self, removed: Q) -> Option<(K,T)>
    {
        self.inner.remove(removed.as_ref())
    }

    /// Prints the trie structure in graphviz format.
    ///
    /// If a file name is specified, the graphviz file is generated.
    /// If not, the output is redirected to standard output.
    #[cfg(feature= "graphviz")]
    pub fn generate_graphviz_file(&self, file: Option<&str>) -> std::io::Result<()> { self.inner.generate_graphviz_file(file) }

    /// Generates the trie structure in a pdf file using `dot` command.
    ///
    /// If a file name is specified, the pdf file is generated.
    /// If not, the output is redirected to standard output.
    ///
    /// # Panics
    /// Panics if the `dot` command was not found.
    #[cfg(feature= "graphviz")]
    pub fn generate_pdf_file(&self, file: Option<&str>) -> std::io::Result<()> { self.inner.generate_pdf_file(file) }

    #[doc(hidden)]
    #[cfg(target_os = "macos")]
    #[cfg(feature= "graphviz")]
    pub fn open_dot_view(&self) -> std::io::Result<()>
    {
        self.inner.open_dot_view()
    }

    /// Gets the number of entries of this map.
    ///
    /// As a trie always contains a top element (the element associated to the
    /// top/empty domain), the length never equals zero.
    #[inline]
    pub fn len(&self) -> NonZeroUsize
    {
        unsafe {
            NonZeroUsize::new_unchecked(self.inner.len())
        }
    }
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
    /// Returns a reference to the value corresponding to the
    /// longuest parent domain of the supplied FQDN (see [`Self::lookup`]).
    ///
    /// It never fails.
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.lookup(index)
    }
}

impl<I:AsRef<Fqdn>,K:AsRef<Fqdn>,T> IndexMut<I> for FqdnTrieMap<K,T>
{
    /// Returns a mutable reference to the value corresponding to the
    /// longuest parent domain of the supplied FQDN (see [`Self::lookup`]).
    ///
    /// It never fails.
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.lookup_mut(index)
    }
}