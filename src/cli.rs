use clap::{arg, Command};

pub fn cli() -> Command {
  Command::new("qp")
    .about("A fast and simple project creator")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .subcommand(
      Command::new("create")
        .about("Create a new project (using pnpm)")
        .arg(arg!(--npm "Use npm instead of pnpm"))
    )
    // .subcommand(
    //   Command::new("config")
    //     .about("Install packages and configure them")
    //     .arg(arg!(--ts "Use typescript"))
    //     // .arg(arg!(<ES> "Use eslint base (airbnb)"))
    //     .arg_required_else_help(true)
    // )


    // subcommand to clear all
}
