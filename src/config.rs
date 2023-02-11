use std::path::MAIN_SEPARATOR_STR;

use clap::{
    Command,
    Arg,
    ArgAction,
    value_parser,
};

const HELP: &'static str = "
Utility for compacting file system paths.

Examples:
    path-compactor /home/sly_boots/projects/some-weird-experiment
    > ~/p/swe

    path-compactor \\
        --threshold 1 \\
        --symbol '#' \\
        --delimiter '>' \\
        --absolute \\
        /home/sly_boots/projects/some-weird-experiment
    > /h/sb/p/swe
";

pub fn get_config() -> Command {
    Command::new("Path Compactor")
        .about("Utility for compacting file system paths.")
        .long_about(HELP)
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .help("Length in characters for a path fragment to trigger compacting")
                .required(false)
                .value_parser(value_parser!(u16))
                .default_value("4")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .help("Maximum character length to reduce a path fragment to")
                .required(false)
                .value_parser(value_parser!(u16))
                .default_value("3")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("symbol")
                .short('s')
                .long("symbol")
                .help("Symbol to replace the home directory path prefix with")
                .required(false)
                .value_parser(value_parser!(String))
                .default_value("~")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("absolute")
                .short('a')
                .long("absolute")
                .help("Disable replacing the home directory path prefix")
                .required(false)
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("separator")
                .short('d')
                .long("delimiter")
                .alias("separator")
                .help("Custom path delimiter to use when joining fragments")
                .required(false)
                .default_value(MAIN_SEPARATOR_STR)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("path")
                .help("The path to compact. Defaults to the current working directory if omitted")
                .required(false)
                .action(ArgAction::Set)
        )

}
