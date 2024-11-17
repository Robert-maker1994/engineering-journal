//! **Array Manipulation**: Algorithm and problems using arrays
//!
//! Arrays are a data structure which can serve as a static or dynamic container for elements of the same type. They can hold data types such as integers, characters, or floating-point numbers, stored in contiguous memory locations.  
//! 
//! Key Characteristics:
//! - **Indexed**: Each element is identified by an index or subscript, which enables random access and manipulation.
//! - **Fixed Size**: Once an array is created, its size cannot be changed. This means you need to know the number of elements you want to store in advance. However, in most languages, arrays can be dynamic.
//! - **Homogeneous Elements**: All elements in an array are of the same type. This ensures that each element occupies the same amount of memory.
//! - **Contiguous Memory Allocation**: Arrays are stored in contiguous memory locations, which allows for efficient access to elements using their index.
//! - **Common Operations**: Visiting elements, removing elements, finding an element, and adding elements.
//! 

pub struct Arrays;


/// You are given a 0-indexed array of positive integers nums.
/// In one operation, you can swap any two adjacent elements if they have the same number of
/// set bits.
/// You are allowed to do this operation any number of times (including zero).
/// Return true if you can sort the array, else return false.
pub mod can_sort_array;

/// Divides players into pairs such that the sum of skills in each pair is equal.
pub mod divide_players;

/// Maximizes the score by repeatedly extracting the largest element, adding it to the score,
/// and then pushing a new value back into the heap.
pub mod max_kelements;

/// Given an array of integers, find the maximum sum of heights of the buildings.
pub mod max_sum_of_heights;

/// Sorts an array using the median as the pivot.
pub mod median_sort;

/// Prime Subtract operation
pub mod prime_sub_operation; 

/// Determines if the given array is a valid mountain array.
/// A mountain array is defined as an array that:
/// - Has at least 3 elements.
/// - There exists some index `i` (0 < i < arr.length - 1) such that:
///   - `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
///   - `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`
pub mod valid_mountain_array;

pub mod maximum_beauty;

pub mod max_pairings;

///  Find the Power of K-Size Subarrays I
pub mod results_array;

pub mod find_min_length_in_subarray;