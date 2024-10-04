struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort();
        let n = skill.len();
        let target_skill = skill[0] + skill[n - 1];
        let mut total_chemistry = 0;
    
        println!("Sorted skills: {:?}", skill);
        println!("Target skill: {}", target_skill);
    
        for i in 0..n / 2 {
            let pair_sum = skill[i] + skill[n - 1 - i];
            println!("Pair: ({}, {}), Pair sum: {}", skill[i], skill[n - 1 - i], pair_sum);
    
            if pair_sum != target_skill {
                return -1;
            }
    
            total_chemistry += (skill[i] as i64) * (skill[n - 1 - i] as i64);
        }
    
        total_chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let skill = vec![3,4];
        let res = 12;
        assert_eq!(Solution::divide_players(skill), res);
    }

    #[test]
    fn test_2() {
        let skill = vec![3,2,5,1,3,4];
        let res = 22;
        assert_eq!(Solution::divide_players(skill), res);

    }

    #[test]
    fn test_3() {
        let skill = vec![1,1,2,3];
        let res = -1;
        assert_eq!(Solution::divide_players(skill), res);
    }
}

