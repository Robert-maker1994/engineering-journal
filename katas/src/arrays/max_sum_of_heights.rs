use super::Arrays;

impl Arrays {
    pub fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
        let mut sum = 0;
        let sz = heights.len();
        for indx in 0..sz {
            sum = sum.max(Self::find_sum(&heights, indx));
        }
        sum
    }

    fn find_sum(heights: &Vec<i32>, max_indx: usize) -> i64 {
        let mut h = heights.clone();
        let mut sum = 0;
        let sz = h.len();

        for indx in (0..max_indx).rev() {
            println!("first {}", h[indx]);
            if h[indx] > h[indx + 1] {
                h[indx] = h[indx + 1];
            }
        }

        for indx in max_indx + 1..sz {
            if h[indx] > h[indx - 1] {
                h[indx] = h[indx - 1];
            }
        }

        for indx in 0..sz {
            sum += h[indx] as i64;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_sum_of_heights() {
        let heights = vec![5, 3, 4, 1, 1];
        assert_eq!(Arrays::maximum_sum_of_heights(heights), 13);

        let heights = vec![6, 5, 3, 9, 2, 7];
        assert_eq!(Arrays::maximum_sum_of_heights(heights), 22);

        let heights = vec![3, 2, 5, 5, 2, 3];
        assert_eq!(Arrays::maximum_sum_of_heights(heights), 18);
    }
}