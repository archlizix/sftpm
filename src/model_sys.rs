use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io::Result};
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub instances: Vec<SystemModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemModel {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub mount_opts: Vec<String>,
    pub mount_point: String,
    pub auth_method: String,
    pub ssh_key: Option<String>,
    pub cmd_before_mount: Option<String>,
}
pub fn write_config_to_file(path: &str, instance: &SystemModel) -> Result<()> {
    let config = Config {
        instances: vec![instance.clone()],
    };
    let toml_string = toml::to_string(&config).map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("序列化失败: {}", err))
    })?;
    fs::write(path, toml_string).map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("写入失败: {}", err))
    })?;
    println!("配置已写入到 '{}'", path);
    Ok(())
}

fn read_config_from_file(path: &str) -> Result<Config> {
    let contents = fs::read_to_string(path).map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("读取失败: {}", err))
    })?;
    let config: Config = toml::from_str(&contents).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("反序列化失败: {}", err),
        )
    })?;
    Ok(config)
}

impl SystemModel {
    fn new(
        id: String,
        host: String,
        port: u16,
        user: String,
        mount_opts: Vec<String>,
        mount_point: String,
        auth_method: String,
        ssh_key: Option<String>,
        cmd_before_mount: Option<String>,
    ) -> Self {
        Self {
            id,
            host,
            port,
            user,
            mount_opts,
            mount_point,
            auth_method,
            ssh_key: ssh_key.or(Some(String::new())), // 处理 Option<String> 类型
            cmd_before_mount: cmd_before_mount.or(Some(String::new())), // 处理 Option<String> 类型
        }
    }

    pub fn validate(&self) -> (bool, Vec<(&'static str, String)>) {
        let mut errors: Vec<(&'static str, String)> = Vec::new();

        macro_rules! validate {
            ($field: expr, $regex: expr, $err_msg: expr) => {
                if !$regex.is_match(&$field) {
                    errors.push(("field", $err_msg));
                }
            };
        }

        let id_regex = Regex::new(r"^[a-zA-Z0-9\.\-_@]+$").unwrap();
        let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9\.\-]+$").unwrap();
        let username_regex = Regex::new(r"^[a-zA-Z0-9\.\-_\@]+$").unwrap();

        validate!(
            self.id,
            id_regex,
            "IDs can only contain letters, digits, dot, @, dash and underscore.".to_string()
        );
        validate!(
            self.host,
            alphanumeric_regex,
            "Hosts can only contain letters, digits, dot and dash.".to_string()
        );

        if !Self::AUTH_METHODS.contains(&&*self.auth_method) {
            errors.push(("auth_method", "Unknown auth type.".to_string()));
        } else if self.auth_method == Self::AUTH_METHOD_PUBLIC_KEY
            && (self.ssh_key.is_none() || !Path::new(&self.ssh_key.as_ref().unwrap()).exists())
        {
            errors.push(("ssh_key", "Invalid ssh key path.".to_string()));
        }
        validate!(
            self.user,
            username_regex,
            "Usernames can only contain letters, at signs and digits.".to_string()
        );
        if self.port < Self::PORT_RANGE_MIN || self.port > Self::PORT_RANGE_MAX {
            errors.push((
                "port",
                format!(
                    "Ports need to be numbers between {} and {}.",
                    Self::PORT_RANGE_MIN,
                    Self::PORT_RANGE_MAX
                ),
            ));
        }

        (errors.is_empty(), errors)
    }

    pub fn save(&self, envir: std::path::PathBuf) -> std::io::Result<()> {
        println!("{:?}", envir);

        std::fs::create_dir_all(envir.parent().unwrap())?;
        let mut file = File::create(envir)?;
        file.write_all(self.export().unwrap().as_bytes())?;
        Ok(())
    }

    fn export(&self) -> Result<String> {
        let config = Config {
            instances: vec![self.clone()],
        };

        toml::to_string(&config).map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("序列化失败: {}", err))
        })
    }

    pub const PORT_RANGE_MIN: u16 = 0;
    pub const PORT_RANGE_MAX: u16 = 65535;
    pub const SSH_PORT_DEFAULT: u16 = 22;
    pub const AUTH_METHOD_PUBLIC_KEY: &'static str = "publickey";
    pub const AUTH_METHOD_AUTHENTICATION_AGENT: &'static str = "authentication-agent";
    pub const AUTH_METHOD_PASSWORD: &'static str = "password";
    // ... other constants ...
    pub const UNSUPPORTED_MOUNT_OPTS: &'static [&'static str] = &["big_writes"];
    pub const AUTH_METHODS: &'static [&'static str] = &[
        Self::AUTH_METHOD_PUBLIC_KEY,
        Self::AUTH_METHOD_AUTHENTICATION_AGENT,
        Self::AUTH_METHOD_PASSWORD,
    ];
}
