#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate app_dirs;
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate sysfs_gpio;

use app_dirs::{app_root, AppDataType, AppInfo};
use std::collections::HashMap;
use std::path::PathBuf;
use sysfs_gpio::{Direction, Pin};

const APPINFO: AppInfo = AppInfo {
    name: "ledpi-rs",
    author: "Michael Gawlik",
};

#[derive(Serialize, Deserialize)]
struct Config {
    pub pins: HashMap<String, u8>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

fn blink() {}

fn main() {
    let config = read_config(Option::None);
    //rocket::ignite().mount("/", routes![index]).launch();
}

fn read_config(config_file_path_opt: Option<PathBuf>) -> Config {
    //
    let config_file_path = config_file_path_opt.unwrap_or_else(|| {
        app_root(AppDataType::UserConfig, &APPINFO)
            .expect("could not get user configuration directory.")
            .join("config.json")
    });

    let config_file = std::fs::File::open(config_file_path.as_path()).expect(&format!(
        "cound not open config file: {}",
        config_file_path.display()
    ));

    return serde_json::from_reader(config_file).expect(&format!(
        "cound not deserialize config file: {}",
        config_file_path.display()
    ));
}
