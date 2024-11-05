mod utils {
    pub mod generating_proof;
    pub mod lib;
}

use risc0_zkvm::{sha::Digestible};
// use risc0_ethereum_contracts::encode_seal;
use utils::generating_proof::generating_proof;
// use utils::lib::{print_receipt_properties };

use dotenv::dotenv;

use std::env;
use std::sync::Arc;


use ethers::prelude::{abigen, Address, Http, LocalWallet, Provider, SignerMiddleware , Signer, Middleware, TransactionRequest };

abigen!(
    FibonacciVerifier,
    r#"[function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal)]"#,
);

#[tokio::main]
async fn main() {
    // wanted to publish generated receipt to sepolia
    // initialize chain connect cofiguration
    dotenv().ok();
    let rpc_url: String = format!(
        "https://{}.g.alchemy.com/v2/{}",
        env::var("NETWORK").unwrap().as_str(),
        env::var("ALCHEMY_API_KEY").unwrap().as_str()
    );
    let chain_id = env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap(); 

    // Set up providers and user's wallet
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to create provider");
    let private_key = env::var("PRIVATE_KEY_HOLESKY").expect("PRIVATE_KEY must be set");
    let wallet: LocalWallet = private_key.parse::<LocalWallet>().expect("invalid private key").with_chain_id(chain_id) ;

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    let fibonachi_verifier_contract_address: Address = "0x5d3d7bb5228F9Ef3624eB8E43BF6f0a68B5B9848"
        .parse()
        .unwrap();

    let fibonachi_verifier_contract = FibonacciVerifier::new(fibonachi_verifier_contract_address, client.clone());

    // Try using import generating receipt function from another folder
    let receipt = generating_proof().expect("Failed to generating receipt"); // .expect("Failed to generate proof data");   

    let journal = receipt.clone().journal.bytes ; 
    let seal = receipt.clone().claim().unwrap().digest().as_bytes().to_vec();
    let call_data = fibonachi_verifier_contract.verify_and_finalize_fibonachi(seal.into(), journal.into()) ; 
    // println!("Journal : {:?}",journal) ;
    // println!("Seal : {:?}",seal) ;
    // println!("Call data : {:?}",call_data.calldata().unwrap()) ;

    let transaction = TransactionRequest::new()
        .to(fibonachi_verifier_contract_address)
        .data(call_data.calldata().unwrap())
        .gas(300_000u64)
        .chain_id(chain_id);

    let pending_tx = client.send_transaction(transaction, None).await.expect("Failed to send transaction") ;

    let tx_hash = pending_tx.tx_hash() ; 
    
    println!("Transaction hash: {:?}", tx_hash);
}
