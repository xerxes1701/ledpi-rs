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

mod cli;
mod config;
mod server;

fn main() {
    use cli::get_args;
    use cli::Opt::*;
    use cli::OptConfig::*;

    match get_args() {
        Config(c) => match c {
            Init { force } => config::init(force),
            Show => config::show(),
        },
        Start => server::start(),
    }
}

