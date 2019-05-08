extern crate reqwest;

use std::collections::HashMap;
use std::env;

fn help() {
	println!("opsi-rpc <address> <username> <password>");
}


fn parse_cli() -> [String; 3] {
	let args: Vec<String> = env::args().collect();

    match args.len() {
	    4 => {
	        let address = &args[1];
	        let username = &args[2];
	        let password = &args[3];

	        [address.to_string(), username.to_string(), password.to_string()]
	    },
	    _ => {
	        // show a help message
	        help();
	        panic!("invalid arguments");
    	}
    }
}

fn post(address: &str, username: &str, password: &str) -> Result<(), Box<std::error::Error>> {
 	let mut jrpc = HashMap::new();
 	jrpc.insert("id", "1");
 	jrpc.insert("method", "backend_info");
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
	let args: [String; 3] = parse_cli();

	let addr = &args[0];
	let username = &args[1];
	let password = &args[2];

	println!("Connecting to {} as {}", addr, username);

	let _result = post(&addr, &username, &password);
	println!("{:#?}", _result)
}
