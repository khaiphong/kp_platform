use std::collections::HashMap;

#[derive(Debug)]
struct User {
	active: bool,
	username: String,
	email: String,
	id: String,      // Kp.Uuid

	aboutme: String,  
	known_as: String,
	url_workspace: String,           // assigned IPv6
	chip_id: Vec<String>,  // firmware id of IamX
	device: HashMap<String, String>,  // to be replaced by db_map_container	
	password: String,
	recovered: String,
	birthdate: String,
	gender: String,
	ethnicity: String,
	keywords: String,
	home_community: String,
	current_community: String,

//	inner_space: InnerSpace, // struct InnerSpace from kp_pmo/ai
}

// collect required data
fn build_user(username: String, email: String, id: String, aboutme: String, known_as: String,
	url_workspace: String, chip_id: Vec<String>, device: HashMap<String, String>, password: String,
	recovered: String, birthdate: String, gender: String, ethnicity: String, 
	keywords: String, home_community: String, current_community: String) -> User {
	User {
		active: true,
		username,
		email,
		id,
		aboutme,
		known_as,
		url_workspace,
		chip_id,
		device,
		password,
		recovered,
		birthdate,
		gender,
		ethnicity,
		keywords,
		home_community,
		current_community,
//		inner_space: InnerSpace::new(),
	}
}

// look at redb btree.rs for minimum db_map_container

/*
//use nix::mount::MsFlags;
use nix::sys::mman::MsFlags;
use std::path::Path;
use nix::unistd::fork;
use nix::unistd::ForkResult;
use nix::sys::wait::waitpid;
use std::ffi::CString;
use nix::sched::unshare;
use nix::sched::CloneFlags;
use nix::unistd::execvp;
use nix::libc;
 use std::fs;
 use nix::mount::mount;
 

//https://www.linkedin.com/pulse/implementing-application-container-inrust-luis-soares-m-sc-/
fn deploy_container(path: &str) {
    let destination = "./newroot/bin";

    // For simplicity, copy the app to a new_root directory
    let new_root = Path::new("newroot/bin");
    std::fs::create_dir_all(&new_root).expect("Failed to create new root directories.");
    
    let deploy_path = new_root.join(Path::new(path).file_name().unwrap());
    
    std::fs::copy(path, &deploy_path).expect("Failed to deploy the app.");
    
    println!("Deployed to {:?}", deploy_path);
}

unsafe fn run_container(cmd: &str, args: Vec<&str>) {
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            // Parent process waits for the child to finish.
            waitpid(child, None).expect("Failed to wait on child");
        }
        Ok(ForkResult::Child) => {
            // Convert Rust strings to C-style strings for execvp
            let c_cmd = CString::new(cmd).expect("Failed to convert to CString");
            let c_args: Vec<CString> = args.iter()
                .map(|arg| CString::new(*arg).expect("Failed to convert to CString"))
                .collect();
            let c_args_refs: Vec<&std::ffi::CStr> = c_args.iter().map(AsRef::as_ref).collect();

            // Unshare namespaces
            unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS).expect("Failed to unshare");

            // Setup the new filesystem root
            let current_dir = std::env::current_dir().unwrap();
            setup_rootfs(&format!("{}/newroot", current_dir.display()));

            execvp(&c_cmd, &c_args_refs).expect("Failed to execvp");
        }
        Err(err) => eprintln!("Fork failed: {}", err),
    }
}

fn setup_rootfs(new_root: &str) {
    // Change the current directory to the new root
    std::env::set_current_dir(new_root).expect("Failed to change directory to new root");

    // Convert Rust string to C-style string for chroot
    let new_root_c = CString::new(new_root).expect("Failed to convert to CString");

    // Now, use chroot to change the root directory
    unsafe {
        if libc::chroot(new_root_c.as_ptr()) != 0 {
            panic!("chroot failed: {}", std::io::Error::last_os_error());
        }
    }

    // Change directory again after chroot to ensure we're at the root
    std::env::set_current_dir("/").expect("Failed to change directory after chroot");

    // Ensure /proc exists in the new root
    fs::create_dir_all("/proc").expect("Failed to create /proc directory");

    // Mount the /proc filesystem
    if !is_proc_mounted() {
        // Now, mount the /proc filesystem
        mount(
            Some("proc"),
            "/proc",
            Some("proc"),
            MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
            None::<&str>
        ).expect("Failed to mount /proc");
    }
}
*/

