use super::DFS;

impl DFS {

/// Removes subfolders from a list of folder paths.
/// 
/// This function takes a vector of folder paths and removes any subfolders, 
/// returning a vector of the remaining folders. The algorithm works as follows:
/// 
/// 1. Sort the folder paths lexicographically. This ensures that any subfolder 
///    will appear immediately after its parent folder in the sorted list.
/// 2. Initialize an empty vector `result` to store the final list of folders.
/// 3. Set `current_parent` to the first folder in the sorted list.
/// 4. Iterate through the sorted list of folders:
///    - For each folder `f`, check if it starts with the `current_parent` followed by a `/`.
///    - If it does not, it means `f` is not a subfolder of `current_parent`.
///    - Add `f` to the `result` vector and update `current_parent` to `f`.
/// 
/// The function returns the `result` vector containing the folders with subfolders removed.
/// 
/// # Arguments
/// 
/// * `folder` - A vector of strings representing folder paths.
/// 
/// # Returns
/// 
/// * A vector of strings representing the folders with subfolders removed.
/// 


    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();

        let mut result = Vec::new();
        let mut current_parent = folder[0].to_string();

        for f in folder {
            // Check if the current folder is a sub-folder of the current parent folder
            if !f.starts_with(&format!("{}/", current_parent)) {
                result.push(f.clone());
                current_parent = f.clone();
            }
        }

        result
        }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_subfolders() {
        let folder = vec![
            String::from("/a"),
            String::from("/a/b"),
            String::from("/c/d"),
            String::from("/c/d/e"),
            String::from("/c/f"),
        ];
        let expected = vec![
            String::from("/a"),
            String::from("/c/d"),
            String::from("/c/f"),
        ];
        assert_eq!(DFS::remove_subfolders(folder), expected);

        let folder = vec![
            String::from("/a"),
            String::from("/a/b/c"),
            String::from("/a/b/d"),
        ];
        let expected = vec![
            String::from("/a"),
        ];
        assert_eq!(DFS::remove_subfolders(folder), expected);

        let folder = vec![
            String::from("/a/b/c"),
            String::from("/a/b/ca"),
            String::from("/a/b/d"),
        ];
        let expected = vec![
            String::from("/a/b/c"),
            String::from("/a/b/ca"),
            String::from("/a/b/d"),
        ];
        assert_eq!(DFS::remove_subfolders(folder), expected);
    }
}
