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
extern crate sysfs_gpio;

mod cli;
mod config;
mod server;
mod gpio;

fn main() {
    use cli::*;

    match get_args() {
        Opt::Config(c) => match c {
            Config::Init { force } => config::init(force),
            Config::Show => config::show(),
        },
        Opt::Start => server::start(),
        Opt::Set(s) => gpio::set_pin(s.pin, s.state).unwrap(),
    }
}
