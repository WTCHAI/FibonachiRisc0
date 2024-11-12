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
use ethers::types::U256 ; 


abigen!(
    FibonachiVerifierContract,
    r#"[function challenge(address challengerAddr , uint256 fibo_result , bytes calldata seal)]"#,
);


#[tokio::main]
async fn main() {
    dotenv().ok();
    let rpc_url: String = format!(
        "https://{}.g.alchemy.com/v2/{}",
        env::var("NETWORK").unwrap().as_str(),
        env::var("ALCHEMY_API_KEY").unwrap().as_str()
    );
    let chain_id = env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap(); 
    
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to create provider");
    let private_key = env::var("PRIVATE_KEY_HOLESKY").expect("PRIVATE_KEY must be set");
    let wallet: LocalWallet = private_key.parse::<LocalWallet>().expect("invalid private key").with_chain_id(chain_id) ;

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    let fibonachi_verifier_contract_address: Address = "0x206ADBa7b67FF84519Add28bDa6b8b3fE14bd6CA"
        .parse()
        .unwrap();
    let fibonachi_verifier_contract = FibonachiVerifierContract::new(fibonachi_verifier_contract_address, client.clone());

    let receipt = bonsai_proof().await.expect("Failed to generating receipt");
    let journal = receipt.clone().journal ; 
    let seal_snarks = encode_seal(&receipt).unwrap() ; 
    let computed_output : u32 = journal.clone().decode().unwrap();

    let call_data = fibonachi_verifier_contract.challenge(wallet.address() , computed_output.into() ,seal_snarks.into()) ; 
    let transaction = TransactionRequest::new()
        .to(fibonachi_verifier_contract_address)
        .data(call_data.calldata().unwrap())
        .gas(300_000u64)
        .gas_price(ethers::utils::parse_units("20", "gwei").unwrap()) // Adjust the gas price as needed
        .chain_id(chain_id);

    let pending_tx = client.send_transaction(transaction, None).await.expect("Failed to send transaction") ;
    let tx_hash = pending_tx.tx_hash() ; 
    println!("Transaction hash: {:?}", tx_hash);
}