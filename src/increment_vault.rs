use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Signer, read_keypair},
    transaction::Transaction,
};
use std::fs::File;
use std::str::FromStr;

fn main() {
    let rpc = RpcClient::new("https://api.devnet.solana.com");

    let program_id = Pubkey::from_str("2hw2KWzHo6Ca5Xjxf3PwUn9p7JdUuyZHBLNz8SVCAySF")
        .expect("Invalid Program ID");

    let mut file = File::open("vault_account.json").expect("File not loaded");
    let vault = read_keypair(&mut file).expect("fail to read vault");

    let mut file = File::open(std::env::var("HOME").unwrap() + "/.config/solana/id.json")
        .expect("ID not opened");
    let payer = read_keypair(&mut file).expect("Payer read failed");

    let init_ix = Instruction::new_with_bytes(
        program_id,
        // &ix_data,
        &[0],
        vec![
            solana_sdk::instruction::AccountMeta::new(vault.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new(payer.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::system_program::id(),
                false,
            ),
        ],
    );
    let increment_ix = Instruction::new_with_bytes(
        program_id,
        // &ix_data,
        &[1],
        vec![
            solana_sdk::instruction::AccountMeta::new(vault.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new(vault.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(
                solana_sdk::system_program::id(),
                false,
            ),
        ],
    );

    let r_block = rpc
        .get_latest_blockhash()
        .expect("failed getting BlockHash");

    let tx =
        Transaction::new_signed_with_payer(&[init_ix], Some(&payer.pubkey()), &[&payer], r_block);

    match rpc.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("transaction success: {}", sig),
        Err(e) => eprintln!("Line 58, Error with RPC.SEND_AND_CONFIRM: {}", e),
    }

    let r_block_2 = rpc.get_latest_blockhash().unwrap();
    let tx_increase = Transaction::new_signed_with_payer(
        &[increment_ix],
        Some(&payer.pubkey()),
        &[&payer],
        r_block_2,
    );

    match rpc.send_and_confirm_transaction(&tx_increase) {
        Ok(sign) => println!("transaction success: {}", sign),
        Err(e) => eprintln!("Line 67: {}", e),
    }
}
