use std::{path::PathBuf};

pub fn generate_name_dir(path: PathBuf) -> Result<String, String> {
    let binding = path.components().last();

    if let Some(bind) = binding {
        if let std::path::Component::Normal(name) = bind {
            let name_str = name.to_str().unwrap();
        
            let capitalized_name = name_str
                .split('-')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
    
            return Result::Ok(format!("[{}](./{})", capitalized_name, name_str));
        }
    }
    Err("cry cry".to_string())
}


pub fn generate_name_file(path: PathBuf) -> Result<String, String> {
    let parent = path.parent()
        .ok_or("No parent directory")?
        .file_stem().expect("No File Stem")
        .to_str()
        .expect("Error on converting parent path");

    let binding = path.components().last();  
    if let Some(bind) = binding {
        if let std::path::Component::Normal(name) = bind {
            let name_str = name.to_str().unwrap();
            let name_with_removed_suffix = name_str.strip_suffix(".md").expect("hello");
            
            let capitalized_name = name_with_removed_suffix
                .split('_')
                .map(|s| {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
    
            return Result::Ok(format!(" - [{}](./{})", capitalized_name, parent.to_owned() + "/" + name_str));
        }
    }
    Err("cry cry".to_string())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_generate_dir() {
        let path: PathBuf = Path::new("C://Users/levon/dev/katas/src/data-structures-and-algorithms").to_path_buf();
        assert_eq!(generate_name_dir(path), Ok("[Data Structures And Algorithms](./data-structures-and-algorithms/README.md)".to_string()))        
    }

    #[test]
    fn test_generate_file() {
        let path: PathBuf = Path::new("C://Users/levon/dev/katas/src/data-structures-and-algorithms/hash_table.md").to_path_buf();
        
        assert_eq!(generate_name_file(path), Ok("[Hash Table](./data-structures-and-algorithms/hash_table.md)".to_string()))    
    }

}
