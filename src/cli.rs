use structopt::{StructOpt};

#[derive(Debug, StructOpt)]
pub enum Opt {

    /// Initialize or show configuration.
    #[structopt(name="config")]
    Config(OptConfig),

    /// Start the http server.
    #[structopt(name="start")]
    Start,
}

#[derive(Debug, StructOpt)]
pub enum OptConfig {

    ///Show the configuration.
    #[structopt(name="show")]
    Show,

    ///Initialize a default config file.
    #[structopt(name="init")]
    Init {
        ///Override the config file, if it already exists.
        #[structopt(short = "f", long = "force")]
        force: bool,
    },
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
