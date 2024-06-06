use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct SystemModel {
    pub(crate) id: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) mount_opts: Vec<String>,
    pub(crate) mount_point: String,
    pub(crate) auth_method: String,
    pub(crate) ssh_key: Option<String>,
    pub(crate) cmd_before_mount: String,
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
        cmd_before_mount: String,
    ) -> Self {
        SystemModel {
            id,
            host,
            port,
            user,
            mount_opts: mount_opts
                .into_iter()
                .filter(|opt| !Self::UNSUPPORTED_MOUNT_OPTS.contains(&opt.as_str()))
                .collect(),
            mount_point,
            auth_method,
            ssh_key,
            cmd_before_mount,
        }
    }

    pub fn validate(&self) -> (bool, Vec<(&'static str, String)>) {
        let mut errors : Vec<(&'static str, String)> = Vec::new();

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
        let path_buf = PathBuf::from_str(self.mount_point.as_str()).unwrap();
        if !path_buf.exists() || !path_buf.is_dir() {
            errors.push(("field", "Invalid remote mount point.".to_string()));
        }

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

    pub fn save(&self, environment_path: &str) -> std::io::Result<()> {
        let path = format!("{}/{}.conf", environment_path, self.id);
        std::fs::create_dir_all(environment_path)?;
        let mut file = File::create(path)?;
        file.write_all(self.export().as_bytes())?;
        Ok(())
    }

    fn export(&self) -> String {
        // This is a simple serialization example. In a real-world scenario, you might want to use a more robust format like JSON or TOML.
        format!(
            "id: {}\n\
             host: {}\n\
             port: {}\n\
             user: {}\n\
             mount_opts: {}\n\
             mount_point: {}\n\
             auth_method: {}\n\
             ssh_key: {}\n\
             cmd_before_mount: {}\n",
            self.id,
            self.host,
            self.port,
            self.user,
            self.mount_opts.join(","),
            self.mount_point,
            self.auth_method,
            self.ssh_key
                .as_ref()
                .map_or("None".to_string(), |k| k.clone()),
            self.cmd_before_mount,
        )
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
