use std::process;

use sftpm::Config;

fn main()
{
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = sftpm::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
