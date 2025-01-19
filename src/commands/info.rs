use crate::cli::args::Commands;
use crate::commands::author;

pub fn execute(cmd: &Commands, _debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Info = cmd {
        println!("Project: {}", env!("CARGO_PKG_NAME"));
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
        println!("Brought to you by Solexma LLC.");
        println!("Website: https://www.solexma.com");
        println!("GitHub: https://github.com/solexma");
        println!("--------------------------------");
        let _ = author::execute(&Commands::Author, _debug);
    }

    Ok(())
}
