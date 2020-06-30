use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "blinc",
    about = "Blinks USB notifier light with different colors depending on command exit code"
)]
pub(crate) enum Opt {
    Blinc {
        /// Initializes configuration file named .blinc (note the dot)
        #[structopt(short, long)]
        init: Option<String>,

        /// Points to configuration file
        #[structopt(short, long, default_value = ".blinc")]
        config: String,
    },
}
