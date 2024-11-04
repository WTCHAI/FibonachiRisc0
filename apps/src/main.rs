mod utils{ 
    pub mod generating_proof;
    pub mod lib ; 
}
use ethers::core::k256::U256;
use utils::generating_proof::generating_proof ;
use utils::lib::{convertImageToU8,print_receipt_properties} ; 

use methods::FIBONACCI_GUEST_ID;

use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use sha2::{ Digest, Sha256};

use ethers::prelude::{
    abigen,
    SignerMiddleware,
    Provider,
    Http,
    LocalWallet,
    Address,
    Signer
};

abigen!(
    FibonacciVerifier,
    r#"[function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal)]"#,
);

#[tokio::main]
async fn main(){
    // wanted to publish generated receipt to sepolia
    // initialize chain connect cofiguration
    dotenv().ok();
    let rpc_url : String = format!("https://{}.g.alchemy.com/v2/{}",
        env::var("NETWORK").unwrap().as_str(),
        env::var("ALCHEMY_API_KEY").unwrap().as_str()
    );

    // Set up providers and user's wallet 
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to create provider"); 
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let wallet : LocalWallet = private_key.parse().expect("invalid private key");
    
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

    let fibonachi_verifier_contract_address  : Address = "0xc5A836Ec8788Af26C552B6F9690b5D9Ea09dF0fD".parse().unwrap() ; 
    let fibonachi_verifier_contract = FibonacciVerifier::new(fibonachi_verifier_contract_address, client.clone()) ; 

    // Try using import generating receipt function from another folder
    let receipt = generating_proof().expect("Failed to generating receipt") ;// .expect("Failed to generate proof data");
    // print_receipt_properties(&receipt) ;
    
    // let binding = verifier_contract.verify(seal.into(), image_id.into() , journal_digest.into());
    // let transaction = binding.send().await.expect("Transaction failed");
    // println!("Transaction hash: {:?}", transaction);
}