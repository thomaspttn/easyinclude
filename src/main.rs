use clap::{arg, command, Command};
use std::error::Error;
use std::fmt;
use std::fs::{self, DirEntry};
use std::path::Path;
use dirs;

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

// gets the id of the running docker container
// TODO: make more robust, obviously
fn list_docker_containers() -> Result<String> {

    // gets the id of the first running container
    let output = std::process::Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("{{.ID}}")
        .arg("--filter")
        .arg("status=running")
        .output()?;

    let output_string = String::from_utf8(output.stdout)?;

    println!("Docker Containers :: {}", output_string);
    Ok(output_string)
}

fn collect_include_paths(id: &str) -> Result<()> {
    let incl_command = r#"gcc -E -xc++ - -v </dev/null 2>&1 | grep -E '^ /'"#;

    // get include paths from inside container
    let output = std::process::Command::new("docker")
        .args(&["exec", id.trim(), "sh", "-c", incl_command])
        .output()?;
    let output_string = String::from_utf8(output.stdout)?;
    let include_lines = output_string.lines();

    // get local home directory + .easyinclude
    let easyincludedir = dirs::home_dir().unwrap().join(".easyinclude");

    for raw_include_path in include_lines {
        let clean_include_path = raw_include_path
            .split_whitespace()
            .next()
            .map(|s| s.trim().strip_prefix("/usr/").unwrap().to_owned())
            .unwrap();

        println!("--- {}", clean_include_path);
        // let dst_dir = easyincludedir.join(clean_include_path);
        let full_path = easyincludedir.join(&clean_include_path);

        println!("--- {}", full_path.display());








        // let incl_output = std::process::Command::new("docker")
        //     .args(&["cp", id, "sh", "-c", incl_command])
        //     .output()?;
    }

    Ok(())
}

fn init() -> Result<()> {
    let container_id = list_docker_containers()?;
    collect_include_paths(&container_id)?;

    Ok(())
}

fn deinit() -> Result<()> {
    let easyincludedir = dirs::home_dir().unwrap().join(".easyinclude");
    fs::remove_dir_all(easyincludedir)?;
    Ok(())
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
