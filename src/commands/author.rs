use crate::cli::args::Commands;

pub fn execute(cmd: &Commands, _debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Author = cmd {
        println!("Author: MiPnamic Von Wirklichkeit");
        println!("Email: mipnamic@mipnamic.net");
    }
    Ok(())
}
