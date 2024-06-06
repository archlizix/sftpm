use clap::Parser;
mod model;
mod model_sys;
use exitcode;
use model::EnvironmentModel;
use model_sys::SystemModel;
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
        Some(Commands::Setup {
            id,
            host,
            port,
            user,
            mount_opt,
            mount_point,
            auth_method,
            ssh_key,
            cmd_before_mount,
        }) => {
            println!("input setup, id:{}, host:{}, port:{}, user:{}, mount_opt:{}, mount_point:{}, auth_method:{}, ssh_key:{}, cmd_before_mount:{}",
                id, host, port, user, mount_opt, mount_point, auth_method, ssh_key, cmd_before_mount);
            let system_model = SystemModel {
                id: id.to_string(),
                host: host.to_string(),
                port: *port as u16,
                user: user.to_string(),
                mount_opts: vec![mount_opt.to_string()],
                mount_point: mount_point.to_string(),
                auth_method: auth_method.to_string(),
                ssh_key: Some(ssh_key.to_string()),
                cmd_before_mount: cmd_before_mount.to_string(),
            };
            let (is_valid, errors) = system_model.validate();
            if is_valid {
                println!("SystemModel is valid.");
                system_model.save("./config").unwrap();
            } else {
                println!("SystemModel is invalid:");
                for (field, error) in errors {
                    println!("{}: {}", field, error);
                }
                std::process::exit(exitcode::DATAERR);
            }
        }
        None => {}
    }
}
