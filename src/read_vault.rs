use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Signer, read_keypair};
use std::fs::File;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VaultState {
    pub initialized: bool,
    pub counter: u64,
}

fn main() {
    let rpc = RpcClient::new("https://api.devnet.solana.com");

    let mut file = File::open("vault_account.json").expect("cannot open vault keypair");
    let vault = read_keypair(&mut file).expect("failed to load keypair");

    println!("Vault pubkey: {}", vault.pubkey());

    let account_data = rpc
        .get_account(&vault.pubkey())
        .expect("Failed to fetch account");

    let data = &account_data.data;

    let vault_state = VaultState::deserialize(&mut &data[..]).expect("Failed to deserialize");

    println!("Vault State: {:?}", vault_state);
}
