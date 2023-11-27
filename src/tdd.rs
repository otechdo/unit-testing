use std::{
    env::args,
    path::Path,
    process::{exit, Command, ExitCode},
};

fn main() -> ExitCode {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && Path::new(&args[1]).is_dir() {
        assert!(Command::new("cargo")
            .arg("run")
            .current_dir(&args[1])
            .spawn()
            .expect("failed to run test")
            .wait()
            .expect("")
            .success());
        exit(0);
    }
    println!("{} <dir>", args[0]);
    exit(1);
}
