Note: this is a project for me to test the rust programing language,  
I am not sure what to do with it yet  ... 

## Start a bitcoin regtest node

```bash
bitcoind -regtest
bitcoin-cli -regtest getnewaddress
bitcoin-cli -regtest generatetoaddress 101 bcrt1yourownaddress.......
bitcoin-cli -regtest getbalance

```

## bitcoin-tx  

```bash
cargo run  
cargo build  
cargo build --release  

```