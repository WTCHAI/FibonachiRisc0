mod utils{ 
    pub mod GeneratingProof;
}
use utils::GeneratingProof::{generating_receipt} ;

use risc0_zkvm::{ sha::Digestible, Receipt };

use std::env;
use dotenv::dotenv;

use ethers::prelude::{abigen, ContractFactory , Provider, Http, Signer, Wallet};

abigen!(
    FibonachiVerifier,
    r#"[function fibonachiVerify(bytes seal, bytes32 journalDigest)]"#,
) ; 

// #[tokio::main]
fn main(){
    // wanted to publish generated receipt to sepolia
    // Try using import generating receipt function from another folder
    let receipt: Receipt = generating_receipt().unwrap() ;
    // Out put data from from computation that belong to the proof in zkp this is public output
    let journal_digest = receipt.journal.bytes.clone() ; 
    // Cryptographic attest represent as zeroknowledge proof that computation was perform correctly 
    let seal = receipt.claim().unwrap().unwrap().digest().as_bytes().to_vec() ;
    
    // .unwrap().digest().as_bytes().to_vec() ;
    println!("Seal : {:?} ",seal) ;
    println!("Journal Digest : {:?} ",journal_digest) ;
    
    // initialize chain connect cofiguration
    dotenv().ok();
    let rpc_url : String = format!("https://{}.g.alchemy.com/v2/{}",
        env::var("NETWORK").unwrap().as_str(),
        env::var("ALCHEMY_API_KEY").unwrap().as_str()
    ) ;

    // Set up providers and user's wallet 
    // let provider = Provider::<HTTP>::try_connect(url)


}