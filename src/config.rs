use std::path::MAIN_SEPARATOR_STR;

use clap::{
    Command,
    Arg,
    ArgAction, value_parser,
};

pub fn get_config() -> Command {
    Command::new("Path Compactor")
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .required(false)
                .value_parser(value_parser!(u16))
                .default_value("4")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .required(false)
                .value_parser(value_parser!(u16))
                .default_value("3")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("symbol")
                .short('s')
                .long("symbol")
                .required(false)
                .value_parser(value_parser!(String))
                .default_value("~")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("absolute")
                .short('a')
                .long("absolute")
                .required(false)
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("separator")
                .short('d')
                .long("delimiter")
                .alias("separator")
                .required(false)
                .default_value(MAIN_SEPARATOR_STR)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("path")
                .required(false)
                .action(ArgAction::Set)
        )

}
