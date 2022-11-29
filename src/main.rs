use std::process::{Command, exit};
use console::style;
use qp::cli;

fn main() {
  let matches = cli().get_matches();

  let package_manager = "pnpm";
  // let mut delay = Duration::from_millis(500);

  let mut stdout = Command::new(package_manager);

  match matches.subcommand() {
    Some(("create", args)) => {
      let npm = *args.get_one::<bool>("npm").unwrap();

      if npm {
        println!("{}", style("This feature is not supported yet.").yellow());
        // package_manager = "npm";
        // delay = Duration::from_millis(2000);
        exit(0);
      }

      stdout
        .arg("init")
        .spawn()
        .unwrap();
    }

    Some(("config", args)) => {
      let ts = *args.get_one::<bool>("ts").unwrap();

      if !ts {
        println!("{}", style("If you do not want to use typescript, you can init confit it by yourself :)").yellow());
        exit(0);
      }

      stdout
        .arg("i") // install
        .arg("-D")
        .arg("typescript")
        .arg("ts-node-dev")
        .arg("@types/node")
        .spawn()
        .unwrap();
    }

    _ => unreachable!()
  }
}
