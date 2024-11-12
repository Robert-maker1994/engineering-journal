use super::Arrays;

impl Array {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    	let mut low = 0;
        let mut high = items.len();
        let i = 0;
        while low < high {
            let mid = (low + high) / 2;
            println!("{}", mid);
            println!("{}", items[mid]);
            println!("{}", queries[i]);
            i =+ 1;
            // if items[mid] == target {
            //    return Some(mid);
            // } else if items[mid] < target {
            //    low = mid + 1;
            // } else {
            //    high = mid;
            // }
        }
    
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_beauty_1() {
        assert_eq!(Arrays::maximum_beauty([[1,2],[3,2],[2,4],[5,6],[3,5], [1,2,3,4,5,6]]), [2,4,5,5,6,6]);
    }
}