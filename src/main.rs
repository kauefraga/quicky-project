use std::process::Command;
use qp::{cli, Console};

fn main() {
  let matches = cli().get_matches();

  let mut console = Console::new();

  let package_manager = "pnpm";
  // let mut delay = Duration::from_millis(500);

  let mut cmd = Command::new(package_manager);

  console.warn("You must ctrl-c after the execution get done (help me to fix it)");

  match matches.subcommand() {
    Some(("init", args)) => {
      let npm = *args.get_one::<bool>("npm").unwrap();

      if npm {
        // package_manager = "npm";
        // delay = Duration::from_millis(2000);
        console.warn("This feature is not supported yet.").exit(0);
      }

      cmd
        .arg("init")
        .spawn()
        .unwrap();
    }

    Some(("config", args)) => {
      let ts = *args.get_one::<bool>("typescript").unwrap();
      let es = *args.get_one::<bool>("eslint").unwrap();

      if ts {
        cmd
          .args(["install", "-D", "typescript", "ts-node-dev", "@types/node"])
          .spawn()
          .unwrap();
      }

      if es {
        console.warn("You should see https://npm.im/eslint-config-airbnb-base");
        console.warn("And also https://npm.im/eslint-config-airbnb-typescript");

        cmd
          .arg("install")
          .arg("-D")
          .arg("eslint@^8.2.0")
          .arg("eslint-plugin-import@^2.25.2")
          .arg("eslint-config-airbnb-typescript")
          .arg("@typescript-eslint/eslint-plugin@^5.13.0")
          .arg("@typescript-eslint/parser@^5.0.0")
          .spawn()
          .unwrap();
      }

      console.warn("If you do not want to use typescript, you should search for nodemon :)")
        .exit(0);
    }

    _ => unreachable!()
  }
}
