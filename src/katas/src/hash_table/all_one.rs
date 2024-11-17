use std::collections::HashMap;

pub struct AllOne {
    _k: HashMap<String, i32>
}

impl AllOne {
    fn _new() -> Self {
        AllOne {
            _k: HashMap::new()
        }
    }
    
    fn _inc(&mut self, key: String) {
        self._k.entry(key).and_modify(|c| *c += 1).or_insert(1);
    }
    
    fn _dec(&mut self, key: String) {
            if let Some(c) = self._k.get_mut(&key) {
                if *c == 1 {
                    self._k.remove(&key);
                } else {
                    self._k.entry(key).and_modify(|c| *c -= 1).or_insert(1);
                }
            }
        }
    
    fn _get_max_key(&self) -> String {
        let mut k = String::new();
        let mut v = 0;
        for (key, value) in self._k.iter() {
            if *value > v {
                k = key.clone();
                v = *value;
            }
        }
        k
    }
    
    fn _get_min_key(&self) -> String {
        let mut k: String = String::new();
        let mut v = i32::MAX;
        for (key, value) in self._k.iter() {
            if *value < v {
                k = key.clone();
                v = *value;
            }
        }
        k
    }
}

