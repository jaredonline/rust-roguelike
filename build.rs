use std::process::Command;
use std::env;

fn main() {
    println!("foo bar");
    let out_dir = env::var("OUT_DIR").unwrap();
    let libtcod_src_dir = "/Users/jmcfarland/src/libtcod";

    let args = &[
        format!("{}/*.dylib", libtcod_src_dir),
        format!("{}/", out_dir)
    ];
    Command::new("cp").args(args).status().unwrap();

    Command::new("cp").arg(format!("{}/terminal.png", libtcod_src_dir))
                      .arg(format!("{}/../../../", out_dir)).status().unwrap();
}
