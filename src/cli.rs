use regex::Regex;
use std::env;

pub fn fncli() {
    let args: Vec<String> = env::args().collect();
    // let command = args.clone();
    // println!("{:?}", args);
    let peer = &args[1];
    let user = &args[2];
    let pass = &args[3];

    // regex to validate url
    let re =
        Regex::new(r"^(http|https)://[a-zA-Z0-9_]+:[a-zA-Z0-9_]+@[a-zA-Z0-9_]+:[0-9]+$").unwrap();

    //let re = Regex::new(r"^(.*:)//([A-Za-z0-9\-\.]+)(:[0-9]+)?(.*)$").unwrap();
    assert!(re.is_match(peer));
    let caps = re.captures(peer).unwrap();
    let protocol = &caps[1];
    let host = &caps[2];
    let port = &caps[3];
    let path = &caps[4];
    println!("{}", protocol);
    println!("{}", host);
    println!("{}", port);
    println!("{}", path);

    if peer == "peer" {
        println!("peer: {}", peer);
    } else {
        println!("That is not a valid command");
    }

    if user == "user" {
        println!("user: {}", user);
    }

    if pass == "pass" {
        println!("pass: {}", pass);
    };
}

// let peer = args.peer;
// let user = args.user;
// let pass = args.pass;
// let amount = args.amount;
// let address = args.address;
// peer.replace("http://localhost:18332", "http://localhost:18333").to_string();
// user.replace("StandUp", "StandUp2").to_string();
// pass.replace("6305f1b2dbb3bc5a16cd0f4aac7e1eba", "6305f1b2dbb3bc5a16cd0f4aac7e1eba2").to_string();
