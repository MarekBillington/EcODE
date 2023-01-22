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

    dbg!(cmds);

    println!("Hello")

}

