use crate::cli::args::Cli;

pub fn execute(_cli: &Cli) {
    let authors = env!("CARGO_PKG_AUTHORS");
    let author_list: Vec<_> = authors.split(':').collect();
    
    match author_list.len() {
        0 => println!("No authors listed"),
        1 => println!("Author: {}", author_list[0]),
        n => {
            println!("Authors:");
            for (i, author) in author_list.iter().enumerate() {
                println!("  {}. {}", i + 1, author.trim());
            }
            println!("\nTotal: {} contributors", n);
        }
    }
}