use collections::Vec;
use collections::string::{String, ToString};
use core_collections::{HashMap, VecDeque};
use enclave::usercall::UserSlice;

use super::get_time_in_millis;
use super::msg_lib::validate;

pub struct SubscriptionCache {
    /// capacity per subscription
    capacity: usize,
    expiration: u64,
    /// String: msg_type; u64: time; UserSlice: msg; Vec<u8>: MAC
    map: HashMap<String, VecDeque<(u64, UserSlice<u8>, Vec<u8>)>>,
    key: [u8;16],
}

impl SubscriptionCache {
    pub fn new(capacity: usize, expiration: u64, key: [u8;16]) -> SubscriptionCache {
        SubscriptionCache {
            capacity: capacity,
            expiration: expiration,
            map: HashMap::new(),
            key: key,
        }
    }

    pub fn insert(&mut self, msg_type: &str, timestamp: u64, msg: Vec<u8>, mac: Vec<u8>) {
        let key = msg_type.to_string();
        let msg = UserSlice::clone_from(&msg);
        if self.map.contains_key(&key) {
            if let Some(l) = self.map.get_mut(&key) {
                Self::cleanup(l, self.expiration, self.capacity);
                (*l).push_front((timestamp, msg, mac));
            }
        } else {
            let mut list = VecDeque::with_capacity(self.capacity);
                list.push_front((timestamp, msg, mac));
                self.map.insert(key, list);
        }
    }

    /// returns collection from newest to oldest
    pub fn get(&mut self, msg_type: &str, n: Option<usize>) -> Vec<(u64, Vec<u8>, Vec<u8>)> {
        let list = self.map.get_mut(msg_type);
        if list.is_none() { return vec!() }
        let mut list = list.unwrap();

        let n = match n {
            Some(v) => v,
            None => self.capacity,
        };
        let (mut result, mut to_remove) = (vec!(), vec!());
        for (i, item) in list.iter().enumerate() {
            let msg = item.1.to_enclave_vec();
            // if value is valid
            if validate(&item.2, item.0, &msg_type.to_string(), &msg, self.key) {
                result.push((item.0, msg, item.2.clone()));
            } else {
                // remove entry, since data is corrupted
                to_remove.push(i);
            }
            if result.len() == n { break }
        }
        for i in to_remove.iter().rev() {
            list.remove(*i);
        }

        result
    }

    #[allow(dead_code)]
    pub fn get_size_per_entry(&self) -> Vec<(String, usize)> {
        self.map.iter().fold(vec!(), |mut acc, (key, val)| { acc.push((key.clone(), val.len())); acc } )
    }

    fn cleanup(list: &mut VecDeque<(u64, UserSlice<u8>, Vec<u8>)>, expiration: u64, capacity: usize) {
        let time = get_time_in_millis();
        loop {
            match list.back(){
                Some(item) if item.0 + expiration <= time => {},
                _ => { break },
            };
            // UserSlice, i.e. data in untrusted Heap is dropped automatically
            list.pop_back();
        }
        while list.len() >= capacity {
            list.pop_back();
        }
    }

    /// This function is intended for testing purpose.
    #[allow(dead_code)]
    pub fn corrupt_data() {
        // TODO
    }

}
