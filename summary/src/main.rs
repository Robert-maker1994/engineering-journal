use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    println!("!!!!!!!!!!!!!!!!!!!!!");
    if let Err(e) = handle_preprocessing() {
        eprintln!("nooo {}", e);
        std::process::exit(1);
    }}

pub struct SummaryGenerator;

impl Preprocessor for SummaryGenerator {
    fn name(&self) -> &str {
        "summary-generator"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let root = ctx.root.join("src");
        let summary_path = root.join("SUMMARY.md");
        let mut summary = File::create(&summary_path)?;

        // let mut entries: Vec<(String, String)> = Vec::new();
        // process_directory(&root, &mut entries)?;

        // Write the collected entries to the summary file
        writeln!(summary, "# Summary")?;
        // for (path, title) in entries {
            writeln!(summary, "- [data-engineering](C:/Users/levon/dev/katas/src/data-engineering/README.md)")?;
        // }


        Ok(book)
    }
}

fn process_directory(dir: &Path, entries: &mut Vec<(String, String)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let title = generate_name_file(&path);
                    entries.push((path.display().to_string(), title));
                }
            }
        } else if path.is_dir() {
            let title = generate_name_dir(&path);
            entries.push((path.display().to_string(), title));
            process_directory(&path, entries)?;
        }
    }
    Ok(())
}

pub fn handle_preprocessing() -> Result<(), Error> {
    let pre = SummaryGenerator;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book).expect("nooo");

    Ok(())
}

fn generate_name_file(path: &Path) -> String {
    path.file_stem().unwrap().to_string_lossy().into_owned()
}

fn generate_name_dir(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
}