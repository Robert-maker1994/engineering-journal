use clap::{ Arg, ArgMatches, Command};
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::errors::Result;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook_generate_summary::generate_summary;
use std::io;
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
    
    let root = std::env::current_dir()
    .expect("Cannot get current directory ")
    .join("src").join("SUMMARY.md");

    if root.exists()  {
        generate_summary();
    }
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

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        Ok(book)
    }
    
    /// Indicate whether a renderer is supported.  This preprocessor can emit MarkDown so should support almost any
    /// renderer.
    fn supports_renderer(&self, renderer: &str) -> bool {
        false
    }
}

