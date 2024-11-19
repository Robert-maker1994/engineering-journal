use std::{fs::{self, DirEntry, File}, io::{self, Write}, path::{Path, PathBuf}};

use mdbook_backend::{generate_name_dir, generate_name_file};


// Todo - handle root files that are not md 
// Handle sub files of directories. 

fn main() -> io::Result<()> {
    let dir = std::env::current_dir().unwrap();
    let root: PathBuf = dir.ancestors().nth(1).unwrap_or_else(|| Path::new("/")).join("src");
    let summary = File::options().read(true).write(true).append(true).open(root.join("SUMMARY.md")).expect("failed to create file");

    for entry in fs::read_dir(root)? {
        if let Ok(cloned_summary) = summary.try_clone() {
            generate_entry_names(entry?, cloned_summary);
        } else {
            eprintln!("Failed to clone summary file");
        }
    }
    Ok(())
}

fn generate_entry_names(entry: DirEntry, summary: File) {
    let path = entry.path();
    if path.is_file() {
        if let Some(extension) = &path.extension() {
            if *extension == "md" {
                let title = generate_name_file(path.clone());

                match title {
                    Ok(name) => {
                        if let Ok(mut file) = summary.try_clone() {
                            writeln!(file, "{}", name).expect("hello");
                        }
                    }
                    Err(e) => eprintln!("Failed to generate name: {}", e),
                }
        }
           }
        }

    if path.is_dir() {
        let title = generate_name_dir(path.clone());

        match title {
            Ok(name) => {
                if let Ok(mut file) = summary.try_clone() {
                    writeln!(file, " - {}", name).expect("hello");
                }
            }
            Err(e) => eprintln!("Failed to generate name: {}", e),
        }
            let entry = fs::read_dir(path).expect("msg");
            for en in entry {

                generate_entry_names(en.expect("msg"), summary.try_clone().expect("msg"));

            }
            
        } else {
            eprintln!("Failed to clone summary file");
        }

}
