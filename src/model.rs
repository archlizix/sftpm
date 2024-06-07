use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct EnvironmentModel {
    mount_path_base: PathBuf,
    config_path_base: PathBuf,
    config_path_mounts: PathBuf,
}

impl EnvironmentModel {
    pub fn get_config_home() -> Result<PathBuf, Box<dyn std::error::Error>> {
        // 尝试获取XDG_CONFIG_HOME环境变量
        if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
            return Ok(PathBuf::from(xdg_config_home));
        }

        // 如果XDG_CONFIG_HOME不存在，尝试获取HOME环境变量
        let home_dir = match env::var("HOME") {
            Ok(home) => PathBuf::from(home),

            // 在Windows上，HOME可能不存在，尝试使用USERPROFILE
            Err(_) => match env::var("USERPROFILE") {
                Ok(userprofile) => PathBuf::from(userprofile),
                Err(e) => return Err(Box::new(e)),
            },
        };

        // 如果HOME或USERPROFILE存在，则返回.config目录的路径
        Ok(home_dir.join(".config"))
    }

    pub fn new() -> EnvironmentModel {
        let config_home = Self::get_config_home().unwrap_or_else(|e| {
            eprintln!("Error getting configuration home directory: {}", e);
            // 你可以在这里提供一个回退路径，或者让程序退出
            PathBuf::from("/default/path/if/error/.config") // 仅作示例，实际中不应硬编码路径
        });

        let mount_path_base = PathBuf::from("/mnt/sshfs/"); // 在Windows上可能需要更改
        let config_path_base = config_home.join("sftpman");
        let config_path_mounts = config_path_base.join("mounts");

        EnvironmentModel {
            mount_path_base,
            config_path_base,
            config_path_mounts,
            // ... 其他字段和方法
        }
    }

    pub fn get_system_config_path(&self, system_id: &str) -> PathBuf {
        self.config_path_mounts.join(format!("{}.json", system_id))
    }

    pub fn get_system_mount_dest(&self, system_id: &str) -> PathBuf {
        self.mount_path_base.join(system_id)
    }

    // ... 其他方法的实现会省略，因为它们需要更多的上下文和错误处理
    // 示例：检查是否已挂载（简化版，不执行实际检查）

    pub fn is_mounted(&self, _system_id: &str) -> bool {
        // 这里应该实现实际检查逻辑
        false // 假设没有挂载任何系统
    }

    // ... 其他方法（get_available_ids, get_mounted_ids, perform_preflight_check等）的实现会类似
}
