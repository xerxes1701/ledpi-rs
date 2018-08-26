use structopt::{StructOpt};

#[derive(Debug, StructOpt)]
pub enum Opt {

    /// Initialize or show configuration.
    #[structopt(name="config")]
    Config(Config),

    /// Start the http server.
    #[structopt(name="start")]
    Start,

    /// Sets a GPIO pin to high or low
    #[structopt(name="set")]
    Set(Set),
}

#[derive(Debug, StructOpt)]
pub enum Config {

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

arg_enum! {
    #[derive(Debug)]
    pub enum GpioState {
        High,
        Low,
    }
}

#[derive(StructOpt, Debug)]
pub struct Set{

    /// the pin's number
    #[structopt()]
    pub pin: u8,

    /// the pin's state
    #[structopt(raw(possible_values = "&GpioState::variants()", case_insensitive = "true"))]
    pub state: GpioState,
}

pub fn get_args() -> Opt {
    Opt::from_args()
}