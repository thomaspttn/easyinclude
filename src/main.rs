use std::fs;
use clap::{arg, command, Command};

fn status() {
    println!("The status is... we're chilling duh")
}

fn collect_include_paths() -> Result<(), std::io::Error> {
    let incl_command = "gcc -E -xc++ - -v </dev/null 2>&1 | grep -E '^ /'";

    let output = std::process::Command::new("docker")
        .args(&["exec", "id", "sh", "-c", incl_command])
        .output()?;

    let include_lines = String::from_utf8_lossy(&output.stdout).lines();

    for includepath in include_lines {
        let incl_output = std::process::Command::new("docker")
            .args(&["cp", "id", "sh", "-c", incl_command])
            .output()?;
    }
        


    Ok(())
}

fn init() {
    fs::create_dir("~/.easyinclude");
}

fn deinit () {

}

fn main() {
    let matches = command!() 
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("status")
                .about("Displays current `easyinclude` status")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes the `easyinclude` process")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("deinit")
                .about("Tears down all `easyinclude` files")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("status", _sub_matches)) => {
            println!("status... nice");
        },
        Some(("init", _sub_matches)) => {
            println!("init... nice");
        },
        Some(("deinit", _sub_matches)) => {
            println!("deinit... nice");
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
