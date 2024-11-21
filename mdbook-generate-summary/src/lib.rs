pub mod summary_generator;
use std::{
    fs::{metadata, read_dir, File}, 
    io::{Result, Write}, 
    path::{PathBuf, Path}
};

pub struct SummaryContent<'a> {
    root: &'a PathBuf
}

impl<'a> SummaryContent<'a> {
    pub fn new(root: &'a PathBuf) -> Self {
        SummaryContent {
            root
        }
    }
}

impl<'a> SummaryContent<'a>{
    pub fn generate_summary(&self) {
    let mut entries: Vec<String> = Vec::new();

    process_directory(&self.root, &mut entries)
    .expect("Failed to process directory");

    entries.sort();

    write_summary(entries, &self.root)
    .expect("Error writing Summary MD");   
        }
}


fn process_directory(dir: &Path, entries: &mut Vec<String> ) -> Result<()>  {

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            entries.push(format_path(&path));   
        } else if path.is_dir() {
            entries.push(format_path(&path));
            process_directory(&path, entries)?;
        }
    }
    Ok(()) 
}


fn is_file_empty(path: &Path) -> Result<bool> {
    let metadata = metadata(path)?;
    Ok(metadata.len() == 0)
}

fn write_summary(folders: Vec<String>, root: &Path ) -> Result<()> {
    if !is_file_empty(&root.join("SUMMARY.md"))? {
        return Ok(());
    }

    let mut summary = File::create(&root.join("SUMMARY.md"))
        .expect("Cannot create summary file");


    writeln!(summary, "# Summary").expect("Failed to write to summary file");

    for path in folders {
    let indent_level = path.matches('/').count();
    let indent = "  ".repeat(indent_level);

    if path.ends_with(".md") && !path.contains("README") {
        writeln!(summary, "{}- [{}]({})", indent, generate_name_file(Path::new(&path)), path).expect("Failed to write to summary file");

    } else {

        if !path.ends_with("rs") && !path.contains("README") {
            writeln!(summary, "{}- [{}]({}/README.md)", indent, generate_name_file(Path::new(&path)), path).expect("Failed to write to summary file");
            }
        }
    }

    Ok(())
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn generate_name_file(path: &Path) -> String {
    path.file_stem()
        .expect("Failed to find file stem")
        .to_str()
        .map(|f| {
            f.replace("_", " ")
            .replace("-", " ")
                .split_whitespace()
                .map(capitalize_first_letter)
                .collect::<Vec<String>>()
                .join(" ")
        })
        .unwrap_or_default()
}

fn format_path(path: &Path) -> String {
    let base_path = path.ancestors().find(|p| p.ends_with("src")).unwrap_or(path);
    let relative_path = path.strip_prefix(base_path).unwrap_or(path);
    format!("./{}", relative_path.to_str().map(|f| f.replace("\\", "/")).expect("error"))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn generate_name_file_test() {
        assert_eq!(generate_name_file(Path::new("binary_tree")), "Binary Tree".to_string());
        assert_eq!(generate_name_file(Path::new("trees")), "Trees".to_string());
        assert_eq!(generate_name_file(Path::new("maxium_subarray_sub")), "Maxium Subarray Sub".to_string());
    }
    
    #[test]
    fn format_path_test() {
        assert_eq!(format_path(Path::new("C:/Users/levon/dev/katas/src/data-engineering/README.md")), "./data-engineering/README.md".to_string());
        assert_eq!(format_path(Path::new("C:/Users/levon/dev/katas/src/README.md")), "./README.md".to_string());
        assert_eq!(format_path(Path::new("C:/Users/levon/dev/katas/src")), "./".to_string());
        assert_eq!(format_path(Path::new("C:/Users/levon/dev/katas/src/data-engineering")), "./data-engineering".to_string());
    }

    #[test]
    fn test() {
        assert_eq!(format_path(Path::new("C:/Users/levon/dev/katas/src/data-engineering/README.md")), "./data-engineering/README.md".to_string())
    }

    #[test]
    fn capitalize_first_letter_test() {
        assert_eq!(capitalize_first_letter("hello"), "Hello".to_string());
        assert_eq!(capitalize_first_letter("rust"), "Rust".to_string());
        assert_eq!(capitalize_first_letter(""), "".to_string());
        assert_eq!(capitalize_first_letter("a"), "A".to_string());
        assert_eq!(capitalize_first_letter("1test"), "1test".to_string());
    }
}

