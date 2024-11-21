use apps::utils::generating_proof::{normal_proof, bonsai_proof} ; 
use apps::utils::receipt_interface::JournalPayload ; 
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{sha::Digestible};

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use hex;
use tokio ; 

use ethers::{abi::encode, prelude::{abigen, Address, Http, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, TransactionRequest}};
use alloy_primitives::U256;

use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    sol_types::SolValue,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let chain_id = env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap(); 
    let private_key = env::var("PRIVATE_KEY_HOLESKY").expect("PRIVATE_KEY must be set");
    let wallet: LocalWallet = private_key.parse::<LocalWallet>().expect("invalid private key").with_chain_id(chain_id) ;

    let receipt = bonsai_proof().await.expect("Failed to generating receipt") ;
    let journal = receipt.clone().journal ;
    let seal_snarks = encode_seal(&receipt).unwrap() ;
    let public_output = U256::abi_decode(&journal.bytes, true).unwrap() ;

    println!("Public output : {:?}", public_output) ; 
    println!("Seal snarks : {:?}",seal_snarks) ;
    println!("Journal (Hex): 0x{}", hex::encode(&journal));
    println!("Seal (Hex): 0x{}", hex::encode(&seal_snarks));

    let journal_payload  = JournalPayload { 
        challenger : wallet.address() ,
        journal :  journal.bytes
    } ; 
    println!("payload : {:?}",journal_payload) ; 
}