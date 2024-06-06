use clap::Parser;
mod model;
use model::EnvironmentModel;
use sftpm::{Cli, Commands};

fn main() {
    let env = EnvironmentModel::new();

    println!("{:?}", env); // 打印EnvironmentModel实例以验证其创建

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
        Some(Commands::Setup{id, host, port, user, mount_opt, mount_point, auth_method, ssh_key, cmd_before_mount}) => {
            println!("input setup, id:{}, host:{}, port:{}, user:{}, mount_opt:{}, mount_point:{}, auth_method:{}, ssh_key:{}, cmd_before_mount:{}",
                id, host, port, user, mount_opt, mount_point, auth_method, ssh_key, cmd_before_mount);
        }
        None => {}
    }
}
