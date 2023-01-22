use std::process::Command;
use std::io::{self, Write};

fn main() {

    let c = cfg!(unix);

    let output = Command::new("sh")
        //.args(["-c"])
        .args(["perf"])
        .output()
        .expect("failed to execute process");

    println!("{:?}", c);
    io::stdout().write_all(&output.stdout).unwrap();
}

