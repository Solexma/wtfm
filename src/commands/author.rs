use crate::cli::args::Commands;
use std::error::Error;

pub fn execute(cmd: &Commands, _debug: bool) -> Result<(), Box<dyn Error>> {
    if let Commands::Author { project_folder } = cmd {
        println!("Managing authors for project in: {:?}", project_folder);
        println!("Author: MiPnamic Von Wirklichkeit");
        println!("Email: mipnamic@mipnamic.net");
        println!("GitHub: https://github.com/MiPnamic");
        println!("Twitter: https://x.com/MiPnamic");
    }

    Ok(())
}
