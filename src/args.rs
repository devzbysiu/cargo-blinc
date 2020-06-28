use clap::crate_authors;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use log::debug;

pub(crate) fn parse_args<'a>() -> ArgMatches<'a> {
    let arguments = App::new("blinc")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Blinks USB notifier light with different colors depending on command exit code")
        // this subcommand is only because of how cargo runs custom commands:
        // cargo blinc --init == cargo-blinc blinc --init
        .subcommand(
            App::new("blinc")
            .version(crate_version!())
            .author(crate_authors!())
            .about("Blinks USB notifier light with different colors depending on command exit code")
            .arg(
                Arg::with_name("init")
                    .help("Initializes configuration file named .blinc (note the dot)")
                    .short("i")
                    .long("init")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("config")
                    .help("Points to configuration file")
                    .short("c")
                    .long("config")
                    .takes_value(true)
                    .default_value(".blinc"),
            ),
        )
        .get_matches();
    let arguments = arguments
        .subcommand_matches("blinc")
        .expect("blinc subcommand should be present");
    debug!("arguments: {:?}", arguments);
    arguments.clone()
}
