/// heavily inspired by Crate lru_time_cache

extern crate slab;
// extern crate time;

// use std::collections::{HashMap, HashSet, VecDeque};
// use std::hash::Hash;
// use time::{Duration, get_time, Timespec};

// use self::slab::{Slab};

// / A data structure for a cache with a specified capacity, implementing the LRU eviction strategy.
// / Values can be inserted with an absolute expiration time, a relative time-to-live or without
// / expiration. The current implementation uses a slab for outsourcing the values. Applying the
// / cache to SGX will use unsecured memory instead.
// / The actual memory (EPC) size needs to be calculated by the admin.
// / growable size in EPC: capacity * (2*size_of_val(K)[list, Map] + 8[slab indexes] + max_size_of(V)] + 8[Map:usize] + 24[Option<Timespec>])
// max size_of(V): e. g. Vec<u8>: 24[Vec] + max_size * 8


// pub struct CacheDS<K, V> { // 136 bytes
//     capacity: usize,    // 8 bytes
//     map: HashMap<K, (usize, Option<Timespec>)>, // 40 bytes
//     list: VecDeque<K>, // 40
//     slab: Slab<V, usize>, // 48
// }
//
// impl<K, V> CacheDS<K, V> where K: PartialOrd + Ord + Clone + Hash {
//     pub fn new(capacity: usize) -> CacheDS<K, V> {
//         CacheDS {
//             capacity: capacity,
//             map: HashMap::with_capacity(capacity),
//             list: VecDeque::new(),
//             slab: Slab::new(capacity),  // TODO remove slab from CacheDS, put it into lib, i.e. lib decides whethet slab, unsafe memory or whatever
//         }
//     }
//
//     fn insert(&mut self, key: K, value: V, expiry: Option<Timespec>) -> Result<(), &str> {
//         self.remove(&key);
//
//         // TODO delete expired values, check for size and in case remove with lru
//         self.remove_expired();
//         // while not_enough_space {
//         if self.capacity == self.map.len() {
//             self.remove_lru();
//         }
//
//         match self.slab.insert(value) {
//             Ok(idx) => { self.map.insert(key.clone(), (idx, expiry));
//                             self.list.push_back(key);
//                             Ok(())},
//             Err(_) => Err("An error ocured while storing value."),
//         }
//
//     }
//
//     pub fn insert_with_expiry(&mut self, key: K, value: V, expiry: Timespec) -> Result<(), &str> {
//         self.insert(key, value, Some(expiry))
//     }
//
//     pub fn insert_with_ttl(&mut self, key: K, value: V, ttl: i64) -> Result<(), &str> {
//         let expiry = get_time() + Duration::milliseconds(ttl);
//         self.insert(key, value, Some(expiry))
//     }
//
//     pub fn insert_without_expiry(&mut self, key: K, value: V) -> Result<(), &str> {
//         self.insert(key, value, None)
//     }
//
//     pub fn remove(&mut self, key: &K) -> Option<V> {
//         if self.contains_key(key) {
//             // self.list.retain(|k| k < key || k > key); // iterating all elements is necassary when keys can be contained multiple times
//             Self::remove_from_list(&mut self.list, key);
//             match self.map.remove(&key) {
//                 Some((idx, _)) => { self.slab.remove(idx) },
//                 None => None,
//             }
//         } else {
//             None
//         }
//     }
//
//     pub fn contains_key(&self, key: &K) -> bool {
//         self.map.contains_key(key)
//     }
//
//     fn remove_from_list(list: &mut VecDeque<K>, key: &K) {
//         match list.iter().position(|k| !(k < key || k > key)) {
//             Some(index) => { list.remove(index); },
//             None => { },
//         }
//     }
//
//     fn is_expired(value: Option<Timespec>, now: Timespec) -> bool {
//         match value {
//             None => false,
//             Some(expiry) => expiry <= now,
//         }
//     }
//
//     /// Since get has to be fast, the removal of expired value isn't triggered here.
//     pub fn get(&mut self, key: &K) -> Option<&V> {
//         let result = self.map.get(key);
//         match result {
//             Some(x) if !Self::is_expired(x.1, get_time()) => {
//                 Self::remove_from_list(&mut self.list, key);
//                 self.list.push_back(key.clone());
//                 self.slab.get(x.0)
//             },
//             _ => None,
//         }
//     }
//
//     fn remove_lru(&mut self) {
//         match self.list.pop_front() {
//             Some(key) => {
//                 match self.map.remove(&key) {
//                     Some((idx, _)) => { self.slab.remove(idx); }
//                     None => {},
//                 }
//             },
//             None => {},
//         }
//     }
//
//     pub fn remove_expired(&mut self) {
//         let now = get_time();
//         let mut expired = HashSet::new();
//         for (k, v) in self.map.iter() {
//             if Self::is_expired(v.1, now) { expired.insert(k.clone()); }
//         }
//         for key in expired.clone() {
//             match self.map.remove(&key) {
//                 Some((idx, _)) => { self.slab.remove(idx); }
//                 None => {},
//             }
//         }
//         // self.map.shrink_to_fit();
//         self.list.retain(|k| !expired.contains(k));
//         // self.list.shrink_to_fit();
//     }
//
// }
//
//
// #[cfg(test)]
// mod test {
//
//     use std::thread::sleep;
//     use std::time::{Duration};
//     use std::mem::{size_of, size_of_val};
//     use super::slab::Slab;
//     use super::time::Timespec;
//
//     fn inserts(cache: &mut super::CacheDS<String, Vec<u8>>) {
//         cache.insert_without_expiry("a".to_string(), "aa".to_string().into_bytes());
//         cache.insert_with_ttl("b".to_string(), "bb".to_string().into_bytes(), 100);
//     }
//
//
//     // #[test]
//     // fn test_size() {
//     //     let mut cache = super::CacheDS::<String, Vec<u8>>::new(2);
//     //     inserts(&mut cache);
//     //     let val = size_of_val(&cache);
//     //     let t = size_of::<Option<Timespec>>();
//     //     println!("size of CacheDS: {}", val);
//     //     assert_eq!(val, 0);
//     // }
//
//     #[test]
//     fn test_inserts() {
//         let mut cache = super::CacheDS::<String, Vec<u8>>::new(2);
//         inserts(&mut cache);
//         let a = "a".to_string();
//         let b = "b".to_string();
//         assert!(cache.contains_key(&a));
//         assert!(cache.contains_key(&b));
//         assert_eq!(cache.get(&a), Some(&"aa".to_string().into_bytes()));
//
//         assert_eq!(cache.map.len(), cache.list.len());
//         assert_eq!(cache.map.len(), cache.slab.count());
//     }
//
//     // #[test]
//     // #[should_panic]
//     // fn test_insert_too_much() {
//     //     let mut cache = super::CacheDS::<String, Vec<u8>>::new(1);
//     //     inserts(&mut cache);
//     // }
//
//     #[test]
//     fn test_removes() {
//         let mut cache = super::CacheDS::<String, Vec<u8>>::new(2);
//         inserts(&mut cache);
//         let key = "a".to_string();
//         cache.remove(&key);
//         cache.insert_without_expiry(key.clone(), key.clone().into_bytes());
//         cache.remove(&key);
//         assert!(!cache.contains_key(&key));
//         assert_eq!(cache.map.len(), cache.list.len());
//         assert_eq!(cache.map.len(), cache.slab.count());
//
//     }
//
//     #[test]
//     fn test_get() {
//         let mut cache = super::CacheDS::<String, Vec<u8>>::new(2);
//         inserts(&mut cache);
//         let a = "a".to_string();
//         let val = "aa".to_string().into_bytes();
//         assert_eq!(cache.get(&a), Some(&val));
//         assert_eq!(cache.get(&a), Some(&val));
//         sleep(Duration::from_millis(110));
//         assert_eq!(cache.get(&("b".to_string())), None);
//         cache.remove(&a);
//         assert_eq!(cache.get(&a), None);
//         assert_eq!(cache.map.len(), cache.list.len());
//         assert_eq!(cache.map.len(), cache.slab.count());
//     }
//
// }
