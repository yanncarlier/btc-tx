use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    // let name = "Brad";
    // let status = "100%";
    let peer = "http://localhost:18332";
    let user = "user";
    let pass = "pass";

    // println!("Command: {}", command);

    if command == "peer" {
        println!("peer: {}", peer);
    } else if command == "user" {
        println!("user: {}", user);
    } else if command == "pass" {
        println!("pass: {}", pass);
    } else {
        println!("That is not a valid command");
    }
}

// let peer = args.peer;
// let user = args.user;
// let pass = args.pass;
// let amount = args.amount;
// let address = args.address;
// peer.replace("http://localhost:18332", "http://localhost:18333").to_string();
// user.replace("StandUp", "StandUp2").to_string();
// pass.replace("6305f1b2dbb3bc5a16cd0f4aac7e1eba", "6305f1b2dbb3bc5a16cd0f4aac7e1eba2").to_string();
