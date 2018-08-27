use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Opt {
    /// Initialize or show configuration.
    #[structopt(name = "config")]
    Config(Config),

    /// Start the http server.
    #[structopt(name = "start")]
    Start,

    /// Sets a GPIO or PWM pin
    #[structopt(name = "set")]
    Set(Set),
}

#[derive(Debug, StructOpt)]
pub enum Config {
    ///Show the configuration.
    #[structopt(name = "show")]
    Show,

    ///Initialize a default config file.
    #[structopt(name = "init")]
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
pub enum Set {
    /// Sets a GPIO pin to 'high' or 'low'
    #[structopt(name = "gpio")]
    Gpio {
        /// the pin's number
        #[structopt(short = "p", long = "pin")]
        pin: u8,

        /// the pin's state
        #[structopt(
            raw(
                possible_values = "&GpioState::variants()",
                case_insensitive = "true"
            ),
            short="s",
            long="state",
        )]
        state: GpioState,
    },

    /// Sets one of the Adafruit-PCA9685 PWM pins
    #[structopt(name = "pwm")]
    Pwm {
        /// the pin's number 0-12
        #[structopt(short = "p", long = "pin")]
        pin: u8,

        /// the on-value 0-255
        #[structopt(long = "on")]
        on: u8,

        /// the off-value 0-255
        #[structopt(long = "off")]
        off: u8,
    },
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
