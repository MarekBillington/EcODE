use std::process::Command;
use std::io::{self, Write};
use std::env;

const PRINT_HELP: &'static str = "
Want some help?
Use the following commands:
    -h  --help  Display help
    -c  --command   Run command, takes <COMMAND> of either Setup|Run
";

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
     * 
     * Need to get a configuration step going and allow commands..?
     */

    let mut cmds: Vec<String> = env::args().collect();
    // Omit the calling command
    cmds.remove(0);


    let output = String::from("echo ") + &String::from(&cmds[1]);
    let proc = Command::new("sh")
        .arg("-c")
        .arg(output)
        .output()
        .expect("failed to execute process");

    dbg!(&cmds);   
        
    // need to remove the first command, then group by actions
    // i.e. -c <COMMAND>
    let out = match cmds[0].as_str() {
        "-h" | "--help" => PRINT_HELP,
        "-c" => "Run the command (Setup or Run)",
        _ => "No command given",
    };

    println!("{}", out);

    io::stdout().write_all(&proc.stdout).unwrap();

}


