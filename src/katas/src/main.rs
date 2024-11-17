use std::collections::HashMap;


fn main() {
    let mut hash = HashMap::with_capacity(10);
    
    hash.insert(100, "Spongebob");
    hash.insert(123, "Patrick");
    hash.insert(321, "Sandy");
    hash.insert(555, "Squidward");
    hash.insert(777, "Gary");
}
