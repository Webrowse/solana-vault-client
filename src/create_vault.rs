
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use dirs;
fn main(){
    let rpc = RpcClient::new("https://api.devnet.solana.com");

    let payer = read_keypair_file(dirs::home_dir().unwrap().join(".config/solana/id.json"))
        .expect("cannot read Solana wallet");
    let program_id = Pubkey::from_str("2hw2KWzHo6Ca5Xjxf3PwUn9p7JdUuyZHBLNz8SVCAySF").unwrap();

    let vault = Keypair::new();

    let mut file = File::create("vault_account.json").unwrap();
    file.write_all(
        &serde_json::to_vec(&vault.to_bytes().to_vec()).unwrap(),
    )
    .unwrap();

        let rent_lamport = rpc.get_minimum_balance_for_rent_exemption(9).unwrap();

        let create_ix = system_instruction::create_account(
            &payer.pubkey(),
            &vault.pubkey(),
            rent_lamport,
            9,
            &program_id,
        );

        let blockhash = rpc.get_latest_blockhash().unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&payer.pubkey()),
            &[&payer, &vault], 
            blockhash);

            let sig = rpc.send_and_confirm_transaction(&tx).unwrap();
            println!("Vault created: {}",vault.pubkey());
            println!("Signature: {}",sig);
}