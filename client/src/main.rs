use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    instruction::{Instruction, AccountMeta},
    transaction::Transaction,
    
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

fn main() {
    // -------------------------------
    // RPC Client (localhost)
    // -------------------------------
    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string()); 
    // let rpc = RpcClient::new("https://api.devnet.solana.com".to_string()); // devnet
    //  let rpc = RpcClient::new("https://api.mainnet-beta.solana.com ".to_string()); // mainnet
     

    
      

    // -------------------------------
    // Generate random keypair instead of reading from file
    // -------------------------------
    let payer = Keypair::new();
    println!("Generated new payer: {}", payer.pubkey());

    // -------------------------------
    // Fund the new keypair (airdrop 1 SOL)
    // -------------------------------
    let airdrop_sig = rpc
        .request_airdrop(&payer.pubkey(), 1_000_000_000) // 1 SOL = 1_000_000_000 lamports
        .expect("Airdrop failed");
    rpc.confirm_transaction( &airdrop_sig)
        .expect("Failed to confirm airdrop");
    println!("Airdropped 1 SOL to new payer");

    // Fully wait for actual finalization
rpc.poll_for_signature( &airdrop_sig).expect("failed to finalize airdrop");

    // -------------------------------
    // Program ID (replace with your program)
    // -------------------------------
    let program_id = Pubkey::from_str("").unwrap(); //@dev ::: add your program_id here


let ix = Instruction {
    program_id,
    accounts: vec![
    ],
    data: vec![0, 1 , 1, 1], //@luqman!_look_man: 0 = deposit // ----///  1, 1, 1 == fake bytes otherwise you will get error since process_instructions parses ix_data[1..]
};

    // // -------------------------------
    // // Build and send transaction
    // // -------------------------------
    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let deposit_sig = rpc
        .send_and_confirm_transaction(&tx)
        .expect("failed to send tx");

    // let sig = rpc.send_transaction(&tx);

    println!("Deposit sent! Signature:");
    // println!("{:?}", sig.unwrap());
    println!("{:?}", deposit_sig);
}
