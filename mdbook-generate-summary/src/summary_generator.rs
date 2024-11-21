use mdbook::errors::Result;
use mdbook::book::Book;
use mdbook::preprocess::PreprocessorContext;
use mdbook::preprocess::Preprocessor;

pub struct SummaryGenerator;

impl Preprocessor for SummaryGenerator {
    fn name(&self) -> &str {
        "summary-generator"
    }
    
    fn run(&self, _: &PreprocessorContext, book: Book) -> Result<Book> {
        Ok(book)
    }
    
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }  
}