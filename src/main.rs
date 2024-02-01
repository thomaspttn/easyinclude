use clap::{arg, command, Command};
use std::error::Error;
use std::fmt;

type Result<T> = std::result::Result<T, EasyIncludeError>;

#[derive(Debug, Clone)]
struct EasyIncludeError {
    details: String
}

impl EasyIncludeError {
    fn new(error: &str) -> Self {
        EasyIncludeError{ details: error.to_string() }
    }
}

impl fmt::Display for EasyIncludeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl<E: Error> From<E> for EasyIncludeError {
    fn from(error: E) -> Self {
        EasyIncludeError::new(&error.to_string())
    }
}

fn status() {
    println!("The status is... we're chilling duh")
}

fn list_docker_containers() -> Result<()> {

    // gets the id of the first running container
    let output = std::process::Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}}")
        .arg("--filter")
        .arg("status=running")
        .output()?;

    let output_string = String::from_utf8(output.stdout)?;

    let x = output_string.lines().next().ok_or("No lines?")?;

    // let output_string = String::from_utf8_lossy(&output.stdout);
    println!("Docker Containers :: {}", x);
    Ok(())
}

fn collect_include_paths() -> Result<()> {
    let incl_command = "gcc -E -xc++ - -v </dev/null 2>&1 | grep -E '^ /'";

    let output = std::process::Command::new("docker")
        .args(&["exec", "id", "sh", "-c", incl_command])
        .output()?;

    let output_string = String::from_utf8_lossy(&output.stdout);
    let include_lines = output_string.lines();

    for includepath in include_lines {
        let incl_output = std::process::Command::new("docker")
            .args(&["cp", "id", "sh", "-c", incl_command])
            .output()?;
    }

    Ok(())
}

fn init() {
    list_docker_containers();
    collect_include_paths();
}

fn deinit() {}

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
            status();
        }
        Some(("init", _sub_matches)) => {
            println!("init... nice");
            init();
        }
        Some(("deinit", _sub_matches)) => {
            println!("deinit... nice");
            deinit();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
