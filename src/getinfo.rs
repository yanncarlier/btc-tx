use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn fngetinfo() {
    let peer = String::from("http://localhost:18332");
    let user = String::from("StandUp");
    let pass = String::from("6305f1b2dbb3bc5a16cd0f4aac7e1eba");

    let rpc = Client::new(&peer, Auth::UserPass(user, pass)).unwrap();

    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:#?}", mining_info);

    let hash = rpc.get_best_block_hash().unwrap();
    let block = rpc.get_block(&hash).unwrap();
    println!("{:?}", block);

    let _ = rpc.stop().unwrap();
}
