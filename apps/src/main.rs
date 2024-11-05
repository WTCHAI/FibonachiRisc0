mod utils {
    pub mod generating_proof;
    pub mod lib;
}

use utils::generating_proof::generating_proof;
use utils::lib::{print_receipt_properties , encode_seal_lib};

use risc0_ethereum_contracts::encode_seal;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use ethers::prelude::{abigen, Address, Http, LocalWallet, Provider, SignerMiddleware};

abigen!(
    FibonacciVerifier,
    r#"[function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal)]"#,
);

#[tokio::main]
async fn main() {
    // wanted to publish generated receipt to sepolia
    // initialize chain connect cofiguration
    dotenv().ok();
    // let rpc_url: String = format!(
    //     "https://{}.g.alchemy.com/v2/{}",
    //     env::var("NETWORK").unwrap().as_str(),
    //     env::var("ALCHEMY_API_KEY").unwrap().as_str()
    // );

    // // Set up providers and user's wallet
    // let provider = Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to create provider");
    // let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    // let wallet: LocalWallet = private_key.parse().expect("invalid private key");

    // let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

    // let fibonachi_verifier_contract_address: Address = "0xBB5F54fB2268152A4A4316D306828B9eC6C0bBbD"
    //     .parse()
    //     .unwrap();

    // let fibonachi_verifier_contract = FibonacciVerifier::new(fibonachi_verifier_contract_address, client.clone());

    // Try using import generating receipt function from another folder
    let receipt = generating_proof().expect("Failed to generating receipt"); // .expect("Failed to generate proof data");   
    // print_receipt_properties(&receipt) ;
    // let journal = receipt.clone().journal.bytes ; 
    // let seal = encode_seal(&receipt);
    // println!("Journal : {:?}",journal) ;
    // println!("Seal : {:?}",seal) ;

    // fibonachi_verifier_contract.verify_and_finalize_fibonachi(seal.into(), journal.into()).await.expect("Failed to verify and finalize fibonachi") ; 
    // println!("Fibonachi verified and finalized on chain") ;
    
}
