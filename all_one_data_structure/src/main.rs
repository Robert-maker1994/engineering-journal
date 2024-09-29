use std::collections::HashMap;

struct AllOne {
    k: HashMap<String, i32>
}

impl AllOne {

    fn new() -> Self {
        AllOne {
            k: HashMap::new()
        }
    }
    
    fn inc(&mut self, key: String) {
        self.k.entry(key).and_modify(|c| *c += 1).or_insert(1);
    }
    
    fn dec(&mut self, key: String) {
            if let Some(c) = self.k.get_mut(&key) {
                if *c == 1 {
                    self.k.remove(&key);
                } else {
                    self.k.entry(key).and_modify(|c| *c -= 1).or_insert(1);
                }
            }
        }
    
    fn get_max_key(&self) -> String {
        let mut k = String::new();
        let mut v = 0;
        for (key, value) in self.k.iter() {
            if *value > v {
                k = key.clone();
                v = *value;
            }
        }
        k
    }
    
    fn get_min_key(&self) -> String {
        let mut k: String = String::new();
        let mut v = i32::MAX;
        for (key, value) in self.k.iter() {
            if *value < v {
                k = key.clone();
                v = *value;
            }
        }
        k
    }
}


fn main() {
    let mut obj = AllOne::new();
     obj.inc("hello".to_string());
     obj.inc("hello".to_string());
        obj.inc("world".to_string());
     let ret_3: String = obj.get_max_key();
     let ret_4: String = obj.get_min_key();
    println!("max key {}", ret_3);
    println!("min key {}", ret_4);
}
