use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use borsh::{BorshSerialize, BorshDeserialize};


use std::str::FromStr;

const PROGRAM_ID: &str = "2hw2KWzHo6Ca5Xjxf3PwUn9p7JdUuyZHBLNz8SVCAySF";

fn main() {
    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(), CommitmentConfig::confirmed()
    );

    let payer = read_keypair_file(
        dirs::home_dir().unwrap().join(".config/solana/id.json")
    ).expect("cannot read Solana wallet");

    println!("Payer: {}", payer.pubkey());

    let program_id = Pubkey::from_str(PROGRAM_ID).expect("Invalid program id");

    println!("Program ID: {}",program_id);

    // let vault = Keypair::new();
    let vault = read_keypair_file("vault_account.json").expect("failed to read");
    println!("Vault account pubkey: {}", vault.pubkey());

    let rent_lamports = rpc.get_minimum_balance_for_rent_exemption(32)
        .expect("failed to get rent");
    println!("Rent-exempt amount: {} lamports", rent_lamports);

    let create_ix = system_instruction::create_account(
        &payer.pubkey(), &vault.pubkey(), rent_lamports, 32, &program_id);

    let init_ix = Instruction{
        program_id,
        accounts: vec![
            AccountMeta::new(vault.pubkey(), false),
        ],
        data: vec![0x00]
    };

    let blockhash = rpc.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_ix, init_ix], 
        Some(&payer.pubkey()), 
        &[&payer, &vault], 
        blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
    println!("Check transaction on Solana Explorer:\nhttps://explorer.solana.com/tx/{}?cluster=devnet",sig);
}