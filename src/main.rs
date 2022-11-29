use std::process::{Command, exit};
use std::time::Duration;
use std::thread;
use console::style;
use quicky_project::cli;

fn main() {
  let matches = cli().get_matches();

  let package_manager = "pnpm";
  let delay = Duration::from_millis(500);

  match matches.subcommand() {
    Some(("create", args)) => {
      let npm = *args.get_one::<bool>("npm").unwrap();

      if npm {
        println!("{}", style("This feature is not supported yet.").yellow());
        // package_manager = "npm";
        // delay = Duration::from_millis(2000);
        exit(0);
      }

      let mut output = Command::new(package_manager)
        .arg("init")
        .spawn()
        .unwrap();

      thread::sleep(delay);

      output.kill().unwrap();
    }

    // Some(("config", args)) => {
    //   let ts = args.get_one::<bool>("npm").unwrap();

    //   if *ts {
    //     Command::new(package_manager)
    //       .arg("i")
    //       .arg("-D")
    //       .arg("typescript @types/node ts-node-dev")
    //       .spawn()
    //       .unwrap();
    //   }

    //   // ...
    // }

    _ => unreachable!()
  }

}
