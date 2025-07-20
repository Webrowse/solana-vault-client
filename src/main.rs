use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Signer, read_keypair_file},
    transaction::Transaction,
};
use solana_sdk::system_program;
use solana_client::rpc_client::RpcClient;
use solana_sdk::system_instruction;

use dirs;
use std::mem;
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct VaultState {
    pub initialized: bool,
    pub _padding: [u8; 7],
    pub counter: u64,
    pub _reserved: [u8; 16],
}

const PROGRAM_ID: &str = "2hw2KWzHo6Ca5Xjxf3PwUn9p7JdUuyZHBLNz8SVCAySF";

fn main() {
    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let payer = read_keypair_file(dirs::home_dir().unwrap().join(".config/solana/id.json"))
        .expect("cannot read Solana wallet");

    println!("Payer: {}", payer.pubkey());

    let program_id = Pubkey::from_str(PROGRAM_ID).expect("Invalid program id");

    println!("Program ID: {}", program_id);

    // let vault = Keypair::new();
    let vault = read_keypair_file("vault_account.json").expect("failed to read");
    println!("Vault account pubkey: {}", vault.pubkey());

    let vault_account_exist = rpc.get_account(&vault.pubkey()).is_ok();
    if vault_account_exist {
        println!("vault exists already, grabbing its content rn");
        match rpc.get_account(&vault.pubkey()) {
            Ok(account) => {
                match VaultState::try_from_slice(&account.data) {
                    Ok(state) => {
                        println!("init: {}, count: {}", state.initialized, state.counter);
                    }
                    Err(e) => {
                        eprintln!("failed to deserialise: {}", e);
                        eprintln!("Account data len :{} vs {} expected VaultState len", account.data.len(), mem::size_of::<VaultState>());
                    }
                };
            }
            Err(e) => {
                eprintln!("Error getting account of vault: {}", e);
            }
        }
    } else {
        let account_size = mem::size_of::<VaultState>() as u64;
        let rent_lamports = rpc
            .get_minimum_balance_for_rent_exemption(account_size as usize)
            .expect("failed to get rent");
        println!(
            "Rent-exempt amount: {} lamports for {} bytes",
            rent_lamports, account_size
        );

        let create_ix = system_instruction::create_account(
            &payer.pubkey(),
            &vault.pubkey(),
            rent_lamports,
            account_size,
            &program_id,
        );

        let init_ix = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(vault.pubkey(), true),
                AccountMeta::new_readonly(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: vec![0x00],
        };

        let blockhash = rpc.get_latest_blockhash().unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[create_ix, init_ix],
            Some(&payer.pubkey()),
            &[&payer, &vault],
            blockhash,
        );

        let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
        println!(
            "Check transaction on Solana Explorer:\nhttps://explorer.solana.com/tx/{}?cluster=devnet",
            sig
        );
    }
}
