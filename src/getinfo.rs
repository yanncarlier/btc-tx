use bitcoincore_rpc::{Auth, Client, RpcApi};

pub fn fngetinfo() {
    let peer = String::from("http://127.0.0.1:18443");
    let user = String::from("user");
    let pass = String::from("pass");

    let rpc = Client::new(&peer, Auth::UserPass(user, pass)).unwrap();

    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:#?}", mining_info);

    let hash = rpc.get_best_block_hash().unwrap();
    let block = rpc.get_block(&hash).unwrap();
    println!("{:?}", block);

    let _ = rpc.stop().unwrap();
}
