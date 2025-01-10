use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};


fn keygen(){
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
    println!(""); println!("To save your wallet, copy and paste the following into a JSON file:"); 
}