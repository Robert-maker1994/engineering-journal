use std::collections::{VecDeque};

use super::Queues;


impl Queues {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let length = students.len(); 
        let mut student_queue: VecDeque<i32> = VecDeque::new();
        let mut sandwich_stack: Vec<i32> = vec![];

        for i in 0..length {
            sandwich_stack.push(sandwiches[length - i - 1]);
            student_queue.push_back(students[i]);
        }

        let mut last_served = 0;

        while !student_queue.is_empty() && last_served < student_queue.len() {
            if *sandwich_stack.last().unwrap() == *student_queue.front().unwrap() {

                sandwich_stack.pop();
                student_queue.pop_front(); 

                last_served = 0;
            } else {

                let student = student_queue.pop_front().unwrap();
                student_queue.push_back(student);
                last_served += 1;
            }
        }

       
        student_queue.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::Queues; 

    #[test]
    fn test_count_students() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        assert_eq!(Queues::count_students(students, sandwiches), 0);

        // let students = vec![1, 1, 1, 0, 0, 1];
        // let sandwiches = vec![1, 0, 0, 0, 1, 1];
        // assert_eq!(Queues::count_students(students, sandwiches), 3);
   
    }
}
