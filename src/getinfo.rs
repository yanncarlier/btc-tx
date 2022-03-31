use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn fngetinfo() {
    let peer = String::from("http://localhost:18332");
    let user = String::from("StandUp");
    let pass = String::from("6305f1b2dbb3bc5a16cd0f4aac7e1eba");

    // if comand line then replace with args
    // let peer = args.peer;
    // let user = args.user;
    // let pass = args.pass;
    // let amount = args.amount;
    // let address = args.address;
    // peer.replace("http://localhost:18332", "http://localhost:18333").to_string();
    // user.replace("StandUp", "StandUp2").to_string();
    // pass.replace("6305f1b2dbb3bc5a16cd0f4aac7e1eba", "6305f1b2dbb3bc5a16cd0f4aac7e1eba2").to_string();

    let rpc = Client::new(&peer, Auth::UserPass(user, pass)).unwrap();

    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:#?}", mining_info);

    let hash = rpc.get_best_block_hash().unwrap();
    let block = rpc.get_block(&hash).unwrap();
    println!("{:?}", block);

    let _ = rpc.stop().unwrap();
}
