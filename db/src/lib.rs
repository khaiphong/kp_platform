use std::collections::HashMap;
// use ai::private::InnerSpace;

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
