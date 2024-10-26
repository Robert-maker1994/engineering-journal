 

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        std::thread::Builder::new()
        .spawn(move ||{
        // Handle negative numbers
        let is_negative = x < 0;
        let mut x = x.abs().to_string().chars().rev().collect::<String>().parse::<i32>().unwrap_or(0);
        
        if is_negative {
            x = -x;
        }
        
        x
        }).unwrap().join().unwrap()
    }
}
