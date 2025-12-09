use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    instruction::{Instruction, AccountMeta},
    transaction::Transaction,
    
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;


////@audit-issue ::  ignore for now
//// crazy errors on local validator
/*
$cargo run >>>>> 

Generated new payer: F6ZwjHHx5vWALyvNAe8ywr1zkTYwwqcm18H3nhfM8oZC
Airdropped 1 SOL to new payer

thread 'main' panicked at client/src/main.rs:68:10:
failed to send tx: Error { request: Some(SendTransaction), kind: RpcError(RpcResponseError { code: -32002, message: "Transaction simulation failed: Attempt to debit an account but found no record of a prior credit.", data: SendTransactionPreflightFailure(RpcSimulateTransactionResult { err: Some(AccountNotFound), logs: Some([]), accounts: None, units_consumed: Some(0), loaded_accounts_data_size: Some(0), return_data: None, inner_instructions: None, replacement_blockhash: None }) }) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/

fn main() {
    // -------------------------------
    // RPC Client (localhost)
    // -------------------------------
    let rpc = RpcClient::new("http://127.0.0.1:8899".to_string());

    // -------------------------------
    // Generate random keypair instead of reading from file
    // -------------------------------
    let payer = Keypair::new();
    println!("Generated new payer: {}", payer.pubkey());

    // -------------------------------
    // Fund the new keypair (airdrop 1 SOL)
    // -------------------------------
    let sig = rpc
        .request_airdrop(&payer.pubkey(), 1_000_000_000) // 1 SOL = 1_000_000_000 lamports
        .expect("Airdrop failed");
    rpc.confirm_transaction(&sig)
        .expect("Failed to confirm airdrop");
    println!("Airdropped 1 SOL to new payer");

    // -------------------------------
    // Program ID (replace with your program)
    // -------------------------------
    let program_id = Pubkey::from_str("Ch3RLCuCkevqL7hwCCdFDetVWz8X9QFbi2J97HSmKYyb").unwrap();

let ix = Instruction {
    program_id,
    accounts: vec![
    ],
    data: vec![0, 1 , 1, 1], //@luqman!_look_man: 0 = deposit // ----///  1, 1, 1 == fake bytes otherwise you will get error since process_instructions parses ix_data[1..]
};

    // -------------------------------
    // Build and send transaction
    // -------------------------------
    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = rpc
        .send_and_confirm_transaction(&tx)
        .expect("failed to send tx");

    println!("Deposit sent! Signature:");
    println!("{}", sig);
}
