use risc0_ethereum_contracts::encode_seal;

use apps::utils::generating_proof::bonsai_proof;


use alloy::{
    network::{EthereumWallet, NetworkWallet}, providers::{Provider, ProviderBuilder}, rpc::types::TransactionRequest, signers::local::PrivateKeySigner, sol_types::SolValue
};
use alloy_primitives::{Address, U256};

use anyhow::Result;
use dotenv::dotenv;
use std::{env, str::FromStr};
use url::Url;

// ABI for the FibonachiVerifier contract
alloy::sol!(
    #[sol(rpc, all_derives)]
    "../hardhat/contracts/IFibonachiVerifier.sol"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_url = format!(
        "https://{}.g.alchemy.com/v2/{}",
        env::var("NETWORK_SEPOLIA").unwrap(),
        env::var("ALCHEMY_API_KEY").unwrap()
    );

    let private_key = env::var("PRIVATE_KEY_2").expect("PRIVATE_KEY must be set") ;
    let signer = PrivateKeySigner::from_str(&private_key).expect("require creting private key signer") ; 
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new()
    .with_recommended_fillers()
    .wallet(wallet.clone())
    .on_http(Url::parse(&rpc_url)?) ; 

    
    let receipt = bonsai_proof().await.expect("Failed to generate receipt");
    let journal = receipt.clone().journal;
    let seal_snarks = encode_seal(&receipt).expect("Failed to encode seal");
    let computed_output = U256::abi_decode(&journal.bytes, true).unwrap() ;

    let fibonachi_verifier_contract_address: Address ="0x4bE39ec6c925954A5Bc95A830814821Ab0E6761D".parse()?;
    let contract = IFibonachiVerifier::new(fibonachi_verifier_contract_address, provider.clone());
    
    let challenge_call = contract.challenge(
        computed_output.into(),
        seal_snarks.into()
    ) ; 
    let calldata = challenge_call.calldata().to_owned() ;

    let tx = TransactionRequest::default()
    .from(wallet.clone().default_signer().address())
    .to(fibonachi_verifier_contract_address)
    .input(calldata.into()) ; 
    
    let tx_hash = provider.send_transaction(tx).await?.watch().await?;
    println!("Transaction hash: {:?}", tx_hash);

    Ok(())
}