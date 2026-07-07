#[cfg(not(feature = "preserve_order"))]
use std::borrow::Borrow;
use std::hash::Hash;

// the concrete backing map, selected by the enabled cargo features
#[cfg(all(not(feature = "preserve_order"), not(feature = "fast_hash")))]
type MapInner<K, V> = std::collections::HashMap<K, V>;
#[cfg(all(not(feature = "preserve_order"), feature = "fast_hash"))]
type MapInner<K, V> = std::collections::HashMap<K, V, rustc_hash::FxBuildHasher>;
#[cfg(all(feature = "preserve_order", not(feature = "fast_hash")))]
type MapInner<K, V> = indexmap::IndexMap<K, V>;
#[cfg(all(feature = "preserve_order", feature = "fast_hash"))]
type MapInner<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;

// backend-specific iterator and entry types, re-exported so the return types of
// `Map`'s methods can be named
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::Entry;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::IntoIter;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::IntoKeys;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::IntoValues;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::Iter;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::IterMut;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::Keys;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::Values;
#[cfg(not(feature = "preserve_order"))]
pub use std::collections::hash_map::ValuesMut;

#[cfg(feature = "preserve_order")]
pub use indexmap::map::Entry;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::IntoIter;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::IntoKeys;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::IntoValues;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::Iter;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::IterMut;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::Keys;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::Values;
#[cfg(feature = "preserve_order")]
pub use indexmap::map::ValuesMut;

/// The map used to store the properties of a [`JsonObject`](crate::JsonObject).
///
/// The backing implementation and hasher are selected by the `preserve_order`
/// and `fast_hash` cargo features, but this type exposes the same API
/// regardless of which are enabled. It's meant to be a drop-in replacement for
/// the standard library's `HashMap`.
pub struct Map<K, V>(MapInner<K, V>);

impl<K, V> Map<K, V> {
  /// Creates an empty map.
  pub fn new() -> Self {
    Map(MapInner::default())
  }

  /// Creates an empty map with at least the specified capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    Map(MapInner::with_capacity_and_hasher(capacity, Default::default()))
  }

  /// Gets the number of entries.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Gets if there are no entries.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Gets the number of entries the map can hold without reallocating.
  pub fn capacity(&self) -> usize {
    self.0.capacity()
  }

  /// Removes all entries.
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// Iterates over the entries.
  pub fn iter(&self) -> Iter<'_, K, V> {
    self.0.iter()
  }

  /// Iterates over the entries with mutable references to the values.
  pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
    self.0.iter_mut()
  }

  /// Iterates over the keys.
  pub fn keys(&self) -> Keys<'_, K, V> {
    self.0.keys()
  }

  /// Iterates over the values.
  pub fn values(&self) -> Values<'_, K, V> {
    self.0.values()
  }

  /// Iterates over mutable references to the values.
  pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
    self.0.values_mut()
  }

  /// Consumes the map, iterating over its keys.
  pub fn into_keys(self) -> IntoKeys<K, V> {
    self.0.into_keys()
  }

  /// Consumes the map, iterating over its values.
  pub fn into_values(self) -> IntoValues<K, V> {
    self.0.into_values()
  }
}

impl<K: Hash + Eq, V> Map<K, V> {
  /// Inserts an entry, returning the previous value for the key if it existed.
  pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    self.0.insert(key, value)
  }

  /// Gets the entry for the given key for in-place manipulation.
  pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
    self.0.entry(key)
  }

  /// Reserves capacity for at least `additional` more entries.
  pub fn reserve(&mut self, additional: usize) {
    self.0.reserve(additional);
  }

  /// Retains only the entries for which the predicate returns `true`.
  pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, f: F) {
    self.0.retain(f);
  }
}

#[cfg(not(feature = "preserve_order"))]
impl<K: Hash + Eq, V> Map<K, V> {
  /// Gets a reference to the value for the given key.
  pub fn get<Q>(&self, key: &Q) -> Option<&V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.get(key)
  }

  /// Gets a mutable reference to the value for the given key.
  pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.get_mut(key)
  }

  /// Gets the key and value for the given key.
  pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.get_key_value(key)
  }

  /// Gets if the map contains the given key.
  pub fn contains_key<Q>(&self, key: &Q) -> bool
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.contains_key(key)
  }

  /// Removes and returns the value for the given key.
  pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.remove(key)
  }

  /// Removes and returns the entry for the given key.
  pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
  where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
  {
    self.0.remove_entry(key)
  }
}

#[cfg(feature = "preserve_order")]
impl<K: Hash + Eq, V> Map<K, V> {
  /// Gets a reference to the value for the given key.
  pub fn get<Q>(&self, key: &Q) -> Option<&V>
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.get(key)
  }

  /// Gets a mutable reference to the value for the given key.
  pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.get_mut(key)
  }

  /// Gets the key and value for the given key.
  pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.get_key_value(key)
  }

  /// Gets if the map contains the given key.
  pub fn contains_key<Q>(&self, key: &Q) -> bool
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.contains_key(key)
  }

  /// Removes and returns the value for the given key, preserving the order of
  /// the remaining entries.
  pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.shift_remove(key)
  }

  /// Removes and returns the entry for the given key, preserving the order of
  /// the remaining entries.
  pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
  where
    Q: Hash + indexmap::Equivalent<K> + ?Sized,
  {
    self.0.shift_remove_entry(key)
  }
}

impl<K, V> Default for Map<K, V> {
  fn default() -> Self {
    Map::new()
  }
}

impl<K: Clone, V: Clone> Clone for Map<K, V> {
  fn clone(&self) -> Self {
    Map(self.0.clone())
  }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for Map<K, V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl<K: Hash + Eq, V: PartialEq> PartialEq for Map<K, V> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<K: Hash + Eq, V: Eq> Eq for Map<K, V> {}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for Map<K, V> {
  fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
    Map(MapInner::from_iter(iter))
  }
}

impl<K: Hash + Eq, V> Extend<(K, V)> for Map<K, V> {
  fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
    self.0.extend(iter);
  }
}

#[cfg(not(feature = "preserve_order"))]
impl<K: Hash + Eq + Borrow<Q>, Q: Hash + Eq + ?Sized, V> std::ops::Index<&Q> for Map<K, V> {
  type Output = V;
  fn index(&self, key: &Q) -> &V {
    &self.0[key]
  }
}

#[cfg(feature = "preserve_order")]
impl<K: Hash + Eq, Q: Hash + indexmap::Equivalent<K> + ?Sized, V> std::ops::Index<&Q> for Map<K, V> {
  type Output = V;
  fn index(&self, key: &Q) -> &V {
    &self.0[key]
  }
}

impl<K, V> IntoIterator for Map<K, V> {
  type Item = (K, V);
  type IntoIter = IntoIter<K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
  type Item = (&'a K, &'a V);
  type IntoIter = Iter<'a, K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl<'a, K, V> IntoIterator for &'a mut Map<K, V> {
  type Item = (&'a K, &'a mut V);
  type IntoIter = IterMut<'a, K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter_mut()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn drop_in_api() {
    let mut map: Map<String, i32> = Map::new();
    assert!(map.is_empty());

    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    *map.entry("c".to_string()).or_insert(0) += 3;
    *map.entry("a".to_string()).or_insert(0) += 10;
    assert_eq!(map.len(), 3);

    // lookups, including the `Index` impl
    assert_eq!(map.get("a"), Some(&11));
    assert_eq!(map["b"], 2);
    assert!(map.contains_key("c"));
    assert_eq!(map.get("missing"), None);

    // mutation
    if let Some(v) = map.get_mut("b") {
      *v = 20;
    }
    assert_eq!(map.get("b"), Some(&20));

    // removal
    assert_eq!(map.remove("c"), Some(3));
    assert_eq!(map.remove("c"), None);
    assert_eq!(map.remove_entry("a"), Some(("a".to_string(), 11)));

    // retain
    map.retain(|_, v| *v > 15);
    assert_eq!(map.len(), 1);
    assert!(map.contains_key("b"));

    // iteration by reference
    let mut count = 0;
    for (_k, _v) in &map {
      count += 1;
    }
    assert_eq!(count, 1);

    // FromIterator + Extend
    let mut other: Map<String, i32> = [("x".to_string(), 1)].into_iter().collect();
    other.extend([("y".to_string(), 2)]);
    assert_eq!(other.len(), 2);
  }
}
