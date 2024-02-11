use clap::{arg, command, Command};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use walkdir::WalkDir;
use dirs;
use serde::{Serialize, Deserialize};

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


#[derive(Serialize, Deserialize, Debug)]
struct CompileCommand {
    directory: String,
    command: Option<String>, 
    arguments: Option<Vec<String>>, 
    file: String,
}

type CompileCommands = Vec<CompileCommand>;



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

fn find_compile_commands() -> Result<Vec<OsString>> {
    let mut return_paths : Vec<OsString> = Vec::new();

    // recursively look for compile commands files
    let compile_commands = OsStr::new("compile_commands.json");
    for entry in WalkDir::new("/Users/thomas/git/modalai")
        .into_iter()
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            if file_name == compile_commands {
                return_paths.push(path.as_os_str().to_os_string());
                println!("Found compile commands at :: {}", path.display());
            }
        }
    }
    Ok(return_paths)
}

fn update_compile_commands(compile_commands_path: &OsStr) -> Result<()> {

    // make a backup of the current compile commands (duh)
    let mut path_string = compile_commands_path.to_os_string();
    path_string.push(".bak");
    let _ = std::fs::copy(compile_commands_path, path_string);
    
    // read JSON into a string
    let json_data = fs::read_to_string(compile_commands_path)?;


    Ok(())
}


fn init() -> Result<()> {

    // extract container ID and use it to collect include paths
    let container_id = list_docker_containers()?;
    collect_include_paths(&container_id)?;

    // gather directories with compile commands for iterative replacement
    let _compile_commands_dirs = find_compile_commands()?;


    Ok(())
}

fn deinit() -> Result<()> {
    let _easyincludedir = dirs::home_dir().unwrap().join(".easyinclude");
    // TODO: REMOVE WITH CAREFUL VERIFICATION
    // fs::remove_dir_all(easyincludedir)?;
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
