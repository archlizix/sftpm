use sftpm::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Mount) => {
            println!("input mount");
        }
        Some(Commands::Umount) => {
            println!("input umount");
        }
        Some(Commands::MountAll) => {
            println!("input mount-all");
        }
        Some(Commands::UmountAll) => {
            println!("input umount-all");
        }
        Some(Commands::Ls) => {
            println!("input ls");
        }
        Some(Commands::Rm) => {
            println!("input rm");
        }
        Some(Commands::PreflightCheck) => {
            println!("input preflight-check");
        }
        Some(Commands::Setup) => {
            println!("input setup");
        }
        None => {}
    }
}
