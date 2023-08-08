# fqdn-trie

[![Crates.io](https://img.shields.io/crates/v/fqdn-trie?style=flat)](https://crates.io/crates/fqdn-trie)
[![Crates.io](https://img.shields.io/crates/d/fqdn-trie?style=flat)](https://crates.io/crates/fqdn-trie)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat)](https://crates.io/crates/fqdn-trie)
[![Docs](https://img.shields.io/docsrs/fqdn-trie)](https://docs.rs/fqdn-trie)

This crate provides two data structures based on FQDN tries in order to provide very fast lookup in
the FQDN hierarchy.

The trie implementation is optimized to FQDN context and follows these rules:
* the search algorithm finds the longuest domain suffix
* the algorithm is case-insensitive
* the internal structure exploits the range of allowed characters in FQDN