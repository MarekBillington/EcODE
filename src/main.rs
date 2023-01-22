use std::process::Command;
use std::io::{self, Write};
use std::env;

fn main() {

    /*
     * Two Ways to capture processes
     * 
     *  1. Instrument
     *      Track the direct requests
     *  2. Sample
     *      Track via already running processes
     * 
     * I think in order to track with voltage and memory usage it needs to be on the instrument
     * Installation and triggering needs to be easy to bundle
     */

    let cmds: Vec<String> = env::args().collect();

    let output = String::from("echo ") + &String::from(&cmds[1]);
    let proc = Command::new("sh")
        .arg("-c")
        .arg(output)
        .output()
        .expect("failed to execute process");

    dbg!(cmds);
    io::stdout().write_all(&proc.stdout).unwrap();

    println!("Hello")

}

