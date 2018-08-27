#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate app_dirs;
extern crate rocket;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
extern crate i2c_pca9685;
extern crate i2cdev;
extern crate sysfs_gpio;

mod cli;
mod config;
mod gpio;
mod pwm;
mod server;

fn main() {
    use cli::*;

    match get_args() {
        Opt::Config(c) => match c {
            Config::Init { force } => config::init(force),
            Config::Show => config::show(),
        },
        Opt::Start => server::start(),
        Opt::Set(s) => match s {
            Set::Gpio { pin, state } => gpio::set_pin(pin, state).unwrap(),
            Set::Pwm { pin, on, off } => pwm::set_pin(pin, on, off),
        },
    }
}
