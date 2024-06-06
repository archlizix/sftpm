use clap::{Parser, Subcommand};
/// A simple sftp helper
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Defines a new sftp file system configuration or edits an old one with the same id
    Setup {
        #[arg(short, long)]
        id: String,
        #[arg(long)]
        host: String,
        #[arg(short, long, default_value_t = 22)]
        port: i32,
        #[arg(short, long)]
        user: String,
        #[arg(long, default_value = "")]
        mount_opt: String,
        #[arg(long)]
        mount_point: String,
        #[arg(short, long, default_value = "password")]
        auth_method: String,
        #[arg(short, long, default_value = "")]
        ssh_key: String,
        #[arg(short, long, default_value = "")]
        cmd_before_mount: String,
    },
    /// Removes a system by id
    Rm,
    /// Detects whether we have everything needed to mount sshfs filesystems
    PreflightCheck,
    /// Lists the available/mounted/unmounted sftp systems
    Ls,
    /// Mounts the specified sftp system, unless it's already mounted
    Mount,
    /// Unmounts the specified sftp system
    Umount,
    /// Mounts all sftp file systems known to sftpman
    MountAll,
    /// Unmounts all sftp file systems known to sftpman
    UmountAll,
}
