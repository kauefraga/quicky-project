use std::process::{Command, exit};
use console::style;
use qp::{cli, warn};

fn main() {
  let matches = cli().get_matches();

  let package_manager = "pnpm";
  // let mut delay = Duration::from_millis(500);

  let mut stdout = Command::new(package_manager);

  match matches.subcommand() {
    Some(("create", args)) => {
      let npm = *args.get_one::<bool>("npm").unwrap();

      if npm {
        // package_manager = "npm";
        // delay = Duration::from_millis(2000);
        warn("This feature is not supported yet.")
      }

      stdout
        .arg("init")
        .spawn()
        .unwrap();
    }

    Some(("config", args)) => {
      let ts = *args.get_one::<bool>("ts").unwrap();

      if !ts {
        warn("If you do not want to use typescript, you can init confit it by yourself :)");
      }

      stdout
        .args(["install", "-D", "typescript", "ts-node-dev", "@types/node"])
        .spawn()
        .unwrap();
    }

    _ => unreachable!()
  }
}
