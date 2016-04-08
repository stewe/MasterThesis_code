/// heavily inspired by Crate lru_time_cache

extern crate time;

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use time::{Duration, get_time, Timespec};

pub struct CacheDS<K, V> {
    map: HashMap<K, (V, Option<Timespec>)>,
    list: VecDeque<K>,
}

impl<K, V> CacheDS<K, V> where K: PartialOrd + Ord + Clone + Hash {
    pub fn new() -> CacheDS<K, V> {
        CacheDS {
            map: HashMap::new(),
            list: VecDeque::new(),
        }
    }

    fn insert(&mut self, key: K, value: V, expiry: Option<Timespec>) -> Option<V> {
        let result = self.remove(&key);
        // TODO delete expired values, check for size and in case remove with lru
        self.map.insert(key.clone(), (value, expiry));
        self.list.push_back(key);
        result

    }

    pub fn insert_with_expiry(&mut self, key: K, value: V, expiry: Timespec) -> Option<V> {
        self.insert(key, value, Some(expiry))
    }

    pub fn insert_with_ttl(&mut self, key: K, value: V, ttl: i64) -> Option<V> {
        let expiry = get_time() + Duration::milliseconds(ttl);
        self.insert(key, value, Some(expiry))
    }

    pub fn insert_without_expiry(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value, None)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.contains_key(key) {
            // self.list.retain(|k| k < key || k > key); // iterating all elements is necassary when keys can be contained multiple times
            Self::find_and_remove(&mut self.list, key);
            self.map.remove(key).map(|(value, _)| value)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn find_and_remove(list: &mut VecDeque<K>, key: &K) {
        match list.iter().position(|k| !(k < key || k > key)) {
            Some(index) => { list.remove(index); },
            None => { },
        }
    }

    fn is_expired(value: &(V, Option<Timespec>), now: Timespec) -> bool {
        match value.1 {
            None => false,
            Some(expiry) => expiry <= now,
        }
    }

    /// Since get has to be fast, the removal of expired value isn't triggered here.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let result = self.map.get(key);
        match result {
            Some(x) if !Self::is_expired(x, get_time()) => {
                Self::find_and_remove(&mut self.list, key);
                self.list.push_back(key.clone());
                Some(&x.0)
            },
            _ => None,
        }
    }

    fn remove_lru(&mut self) {
        match self.list.pop_front() {
            Some(key) => {
                self.map.remove(&key);
            },
            None => {},
        }
    }

    pub fn remove_expired(&mut self) {
        let now = get_time();
        let mut expired = HashSet::new();
        for (k, v) in self.map.iter() {
            if Self::is_expired(v, now) { expired.insert(k.clone()); }
        }
        for key in expired.clone() {
            self.map.remove(&key);
        }
        self.list.retain(|k| !expired.contains(k));
    }

}


#[cfg(test)]
mod test {

    use std::thread::sleep;
    use std::time::Duration;

    fn inserts(cache: &mut super::CacheDS<String, String>) {
        cache.insert_without_expiry("a".to_string(), "aa".to_string());
        cache.insert_with_ttl("b".to_string(), "bb".to_string(), 100);
    }

    #[test]
    fn test_inserts() {
        let mut cache = super::CacheDS::<String, String>::new();
        inserts(&mut cache);
        let a = "a".to_string();
        let b = "b".to_string();
        assert!(cache.contains_key(&a));
        assert!(cache.contains_key(&b));
        assert_eq!(cache.get(&a), Some(&"aa".to_string()));
    }

    #[test]
    fn test_removes() {
        let mut cache = super::CacheDS::<String, String>::new();
        inserts(&mut cache);
        let key = "a".to_string();
        cache.remove(&key);
        cache.insert_without_expiry(key.clone(), key.clone());
        cache.remove(&key);
        assert!(!cache.contains_key(&key));
    }

    #[test]
    fn test_get() {
        let mut cache = super::CacheDS::<String, String>::new();
        inserts(&mut cache);
        let a = "a".to_string();
        let val = "aa".to_string();
        assert_eq!(cache.get(&a), Some(&val));
        assert_eq!(cache.get(&a), Some(&val));
        sleep(Duration::from_millis(110));
        assert_eq!(cache.get(&("b".to_string())), None);
        cache.remove(&a);
        assert_eq!(cache.get(&a), None);
    }

}
