
impl Solution {
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
        assert_eq!(Solution::remove_subfolders(folder), expected);

        let folder = vec![
            String::from("/a"),
            String::from("/a/b/c"),
            String::from("/a/b/d"),
        ];
        let expected = vec![
            String::from("/a"),
        ];
        assert_eq!(Solution::remove_subfolders(folder), expected);

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
        assert_eq!(Solution::remove_subfolders(folder), expected);
    }
}
