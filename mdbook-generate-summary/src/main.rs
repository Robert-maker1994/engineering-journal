use clap::{ Arg, ArgMatches, Command, Subcommand};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::errors::{ Result};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::{io};
// use std::path::Path;
use std::process;

pub fn make_app() -> Command {
    Command::new("generate-summary")
        .about("A mdbook preprocessor that adds support for generate summary pages")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    
    }  else if let Err(e) = handle_preprocessing() {
        println!("errrr {}", e);
        process::exit(1);
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let pre = SummaryGenerator;

    let processed_book = pre.run(&ctx, book)?;


    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let pre = SummaryGenerator::default();

    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

pub struct SummaryGenerator;

impl Default for SummaryGenerator {
    fn default() -> Self {
        SummaryGenerator {   }
    }
}

impl Preprocessor for SummaryGenerator {
    fn name(&self) -> &str {
        "summary-generator"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let root = ctx.root.join("src");
        let summary_path = root.join("SUMMARY.md");
        let mut summary = File::create(&summary_path)?;

        let mut entries: Vec<(String, String)> = Vec::new();
        process_directory(&root, &mut entries)?;
        
        writeln!(summary, "# Summary")?;
        for (path, title) in entries {
            writeln!(summary, " - [{}]({})", title, path)?;
        }

        Ok(book)
    }
    
    /// Indicate whether a renderer is supported.  This preprocessor can emit MarkDown so should support almost any
    /// renderer.
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
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
                    entries.push((generate_path_name(&path), title));
                }
            }
        } else if path.is_dir() {
            let title = generate_name_dir(&path);
            let p = generate_path_name(&path);  
            entries.push((format!(r"{}\README.md", p), title));
            process_directory(&path, entries)?;
        }
    }
    Ok(())
}

fn generate_name_file(path: &Path) -> String {
    path.file_stem().unwrap().to_string_lossy().into_owned()
}

fn generate_name_dir(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
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
    fn test() {
        assert_eq!(generate_path_name(Path::new("C:/Users/levon/dev/katas/src/data-engineering/README.md")), "./data-engineering/README.md".to_string())
    }
}
