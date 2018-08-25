use app_dirs::{app_root, AppDataType, AppInfo};
use toml;
use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf, 
    io::{Read, Write}};

const APPINFO: AppInfo = AppInfo {
    name: "ledpi-rs",
    author: "Michael Gawlik",
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pins: HashMap<String, u8>,
}

pub fn read_config(path: Option<PathBuf>) -> Config {
    //
    let path = path.unwrap_or_else(get_default_path);

    let mut reader = File::open(path.clone())
        .expect(&format!("could not open config file: {}", path.display()));

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)
        .expect(&format!("could not read config: {}", path.display()));

    return toml::from_str(&buffer.as_str())
        .expect(&format!("config file invalid: {}", path.display()));
}

pub fn show() {
    let path = get_default_path();

    println!("default config path: {}", path.display());

    if !path.exists() {
        println!("file not found!");
    } else {
        let config = read_config(Some(path));
        println!("{:?}", config);
    }
}

pub fn init(force: bool) {
    let path = get_default_path();

    if !force && path.exists() {
        println!("config file '{}' already exits. use 'config init --force' to override.", path.display());
        return;
    }

    let mut pins = HashMap::new();
    pins.insert("A".to_string(), 23);
    pins.insert("B".to_string(), 42);

    let config = Config { pins };

    let s = toml::to_string_pretty(&config).unwrap();

    let mut writer = File::create(path.clone())
        .expect(&format!("could not open config file: {}", path.display()));

    writer.write(s.as_bytes())
        .expect(&format!("could not write config: {}", path.display()));

    println!("created default config file: {}", path.display());
}

fn get_default_path() -> PathBuf {
    app_root(AppDataType::UserConfig, &APPINFO)
        .expect("could not get user configuration directory.")
        .join("config.toml")
}
