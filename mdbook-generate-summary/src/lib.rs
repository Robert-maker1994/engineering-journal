use std::{
    fs::{self, File}, 
    io::{self, Write}, 
    path::{Path}
};

pub fn generate_summary() {
    let root = std::env::current_dir()
    .expect("Cannot get current directory ").join("src");

    println!("root {:?}", root);

    let mut summary = File::create(&root.join("SUMMARY.md")).expect("Cannot create summary file");
    
    let mut entries: Vec<(String, String)> = Vec::new();
    process_directory(&root, &mut entries).expect("Failed to process directory");
    
    writeln!(summary, "# Summary").expect("Failed to write to summary file");
    for (path, title) in entries {
        writeln!(summary, " - [{}]({})", title, path).expect("Failed to write entry to summary file");
    }
    
}


fn process_directory(dir: &Path, entries: &mut Vec<(String, String)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && !path.ends_with("README.md") {

            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let title = generate_name_file(&path);
    
                    entries.push((generate_path_name(&path), title));
                }
            }
        } else if path.is_dir() {
            let title = generate_name_file(&path);
            let p = generate_path_name(&path);  
            entries.push((format!(r"{}\README.md", p), title));
            process_directory(&path, entries)?;
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
    path.file_stem().expect("he").to_str().map(|f| {
        let remove_ = f.replace("_", " ");
        remove_.split_whitespace()
        .map(|word| capitalize_first_letter(word))
        .collect::<Vec<String>>()
        .join(" ")
    }).unwrap().to_owned()
}

fn generate_path_name (path: &Path) -> String {
    let base_path = path.ancestors().find(|p| p.ends_with("src")).unwrap_or(path);

       let relative_path = path.strip_prefix(base_path).unwrap_or(path);
   
       format!("./{}", relative_path.display())
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
    fn test() {
        assert_eq!(generate_path_name(Path::new("C:/Users/levon/dev/katas/src/data-engineering/README.md")), "./data-engineering/README.md".to_string())
    }
}
