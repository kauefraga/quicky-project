use clap::{arg, Command};

pub fn cli() -> Command {
  Command::new("Quicky Project")
    .about("A fast and simple nodejs project helper")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .allow_external_subcommands(true)
    .subcommand(
      Command::new("init")
        .about("Initialize a new project (using pnpm)")
        .arg(arg!(--npm "Use npm instead of pnpm"))
    )
    .subcommand(
      Command::new("config")
        .about("Install packages and configure them")
        .arg(arg!(--typescript "Use typescript"))
        .arg(arg!(--eslint "Use eslint base (airbnb)"))
        .arg(arg!(-a --all "Use all configs"))
        .arg_required_else_help(true)
    )

    // subcommand to clear all
}
