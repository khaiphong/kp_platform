/*
https://github.com/luishsr/rust-container
extern crate nix;
extern crate clap;
extern crate libc;

use nix::sched::{unshare, CloneFlags};
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use nix::mount::{MsFlags, mount};
use clap::{App, Arg, SubCommand};
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
*/

fn main() {
    println!("Hello, world!");
    
/*
    let binary_path = "/your/path/here/";  // adjust this path
    let target_dir = "/your/path/here/newroot/bin";  // adjust to your container's `bin` directory

    let matches = App::new("Simple Container CLI")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a command in an isolated container")
                .arg(Arg::with_name("COMMAND").required(true).index(1))
                .arg(Arg::with_name("ARGS").multiple(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("deploy")
                .about("Deploys a file or directory to the container root")
                .arg(Arg::with_name("PATH").required(true).index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let cmd = matches.value_of("COMMAND").unwrap();
        let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
        unsafe { run_container(cmd, args); }
    }  else if let Some(matches) = matches.subcommand_matches("deploy") {
        let path = matches.value_of("PATH").unwrap();
        deploy_container(path);
    }

    let proc_cstring = CString::new("/proc").unwrap();

*/  
}
