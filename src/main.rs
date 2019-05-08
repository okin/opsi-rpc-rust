extern crate reqwest;
extern crate clap;

use clap::{Arg, App};

use std::collections::HashMap;

fn post(address: &str, username: &str, password: &str, method: &str) -> Result<(), Box<std::error::Error>> {
 	let mut jrpc = HashMap::new();
 	jrpc.insert("id", "1");
	jrpc.insert("method", method);
	jrpc.insert("params", "");

	let client = reqwest::Client::builder()
		.danger_accept_invalid_certs(true)
		.build()?;

	let mut response = client.post(address)
 		.basic_auth(username, Some(password))
 		.json(&jrpc)
 		.send()?;
 	println!("Headers: {:#?}", response);

 	let content = response.text()?;
 	println!("Body: {:#?}", content);

 	Ok(())
}

fn main() {
	let matches = App::new("opsi-rpc-rust")
                          .arg(Arg::with_name("addr")
                               .help("Sets an address")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("username")
                               .help("Sets the username")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("password")
                               .help("Sets the password")
                               .required(true)
                               .index(3))
                          .arg(Arg::with_name("method")
                                .help("Method to call")
                                .index(4))
                          .get_matches();


	let addr = matches.value_of("addr").unwrap();
	let username = matches.value_of("username").unwrap();
	let password = matches.value_of("password").unwrap();
	let method = matches.value_of("method").unwrap_or("backend_info");

	println!("Connecting to {} as {} to execute {}", addr, username, method);

	let _result = post(&addr, &username, &password, &method);
	println!("{:#?}", _result)
}
