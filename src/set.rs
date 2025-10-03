use std::iter::FromIterator;
use std::num::NonZeroUsize;
use std::ops::Index;

use crate::{HasFqdn, Fqdn};
use crate::trie::InnerTrie;

/// A set of FQDN based on a suffix trie
pub struct FqdnTrieSet<T:AsRef<Fqdn>> {
    inner: InnerTrie<TrieSetElt<T>>
}

struct TrieSetElt<T:AsRef<Fqdn>>(T);
impl<T:AsRef<Fqdn>> HasFqdn for TrieSetElt<T> {
    #[inline] fn fqdn(&self) -> &Fqdn { self.0.as_ref() }
}

impl<T:AsRef<Fqdn>> FqdnTrieSet<T> {

    /// Creates a new set with the root element.
    ///
    /// # Panics
    /// Panics if the FQDN of the root element is not the empty one (`.`)
    #[inline]
    pub fn new(root: T) -> Self
    {
        Self { inner: InnerTrie::new(TrieSetElt(root)) }
    }

    /// Creates a new set with the root element and an initial capacity.
    ///
    /// # Panics
    /// Panics if the FQDN of the root element is not the empty one (`.`)
    #[inline]
    pub fn with_capacity(root: T, capacity: usize) -> Self
    {
        Self { inner: InnerTrie::with_capacity(TrieSetElt(root), capacity) }
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
    /// use fqdn_trie::FqdnTrieSet;
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    /// fqdns.reserve(100);
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

    /// Gets the number of element of this set.
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

    /// Gets the element, if any, which exactly matches (i.e. equals) the given FQDN.
    ///
    /// To find the longuest parent domain, consider [`Self::lookup`]
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    /// fqdns.insert(fqdn!("orange.com"));
    /// fqdns.insert(fqdn!("mail.orange.com"));
    ///
    /// assert!( fqdns.get(fqdn!("orange.com")).is_some() );
    /// assert!( fqdns.get(fqdn!("www.orange.com")).is_none() );
    /// ```
    #[inline]
    pub fn get<K: AsRef<Fqdn>>(&self, look: K) -> Option<&T>
    {
        self.inner.get_exact_leaf(look.as_ref()).map(|x| &x.0)
    }

    /// Gets an iterator over all the stored FQDN
    pub fn iter(&self) -> impl Iterator<Item=&T>
    {
        self.inner.iter().map(|x| &x.0)
    }

    /// Gets the element which the longuest parent domain of the given FQDN.
    ///
    /// To use an exact match, consider [`Self::get`]
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    /// fqdns.insert(fqdn!("orange.com"));
    /// fqdns.insert(fqdn!("mail.orange.com"));
    ///
    /// assert_eq!( *fqdns.lookup(fqdn!("orange.com")), fqdn!("orange.com") );
    /// assert_eq!( *fqdns.lookup(fqdn!("www.orange.com")), fqdn!("orange.com") );
    /// assert_eq!( *fqdns.lookup(fqdn!("blue.com")), fqdn!(".") );
    /// ```
    #[inline]
    pub fn lookup<K:AsRef<Fqdn>>(&self, look: K) -> &T
    {
        &self.inner.lookup(look.as_ref()).0
    }

    /// Adds a new element in the set.
    ///
    /// Returns `true` if the element is effectively added and `false` if an element with the same
    /// FQDN is already present in the set (which remains unmodified)
    ///
    /// To replace an already present element, consider [`Self::replace`]
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    ///
    /// assert!(fqdns.insert(fqdn!("orange.com")));
    /// assert!( ! fqdns.insert(fqdn!("orange.com")));
    /// ```
    #[inline]
    pub fn insert(&mut self, added: T) -> bool
    {
        self.inner.insert(TrieSetElt(added))
    }

    /// Adds a value to the set, replacing the existing element, if any,
    /// that is associated to the same FQDN. Returns the replaced element.
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    ///
    /// assert!(fqdns.replace(fqdn!("orange.com")).is_none());
    /// assert!(fqdns.replace(fqdn!("orange.com")).is_some());
    /// ```
    #[inline]
    pub fn replace(&mut self, value: T) -> Option<T>
    {
        self.inner.replace(TrieSetElt(value)).map(|x| x.0)
    }

    /// If the set contains an element is to the FQDN, removes it from the set and drops it.
    /// Returns whether such an element was present.
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    /// fqdns.insert(fqdn!("orange.com"));
    ///
    /// assert!(fqdns.remove(fqdn!("orange.com")));
    /// assert!( ! fqdns.remove(fqdn!("orange.com")));
    /// ```
    #[inline]
    pub fn remove<K:AsRef<Fqdn>>(&mut self, removed: K) -> bool
    {
        self.inner.remove(removed.as_ref()).is_some()
    }

    /// Removes and returns the element in the set, if any, that is associated to the FQDN.
    ///
    /// # Example
    /// ```
    /// use fqdn::{fqdn,FQDN};
    /// use fqdn_trie::FqdnTrieSet;
    ///
    /// let mut fqdns = FqdnTrieSet::<FQDN>::default();
    /// fqdns.insert(fqdn!("orange.com"));
    ///
    /// assert!(fqdns.take(fqdn!("orange.com")).is_some());
    /// assert!(fqdns.take(fqdn!("orange.com")).is_none());
    /// ```
    #[inline]
    pub fn take<K:AsRef<Fqdn>>(&mut self, removed: K) -> Option<T>
    {
        self.inner.remove(removed.as_ref()).map(|x| x.0)
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

impl<T:AsRef<Fqdn>+Default> Default for FqdnTrieSet<T>
{
    #[inline]
    fn default() -> Self { Self::new(T::default()) }
}

impl<I:AsRef<Fqdn>,T:AsRef<Fqdn>> Index<I> for FqdnTrieSet<T>
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
