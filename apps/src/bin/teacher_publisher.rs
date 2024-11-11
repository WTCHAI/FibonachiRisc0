// use apps::utils::{generating_proof::fibo_enhance_proof, receipt_interface::PayloadRequest} ; 

// use risc0_zkvm::{sha::Digestible};

// use dotenv::dotenv;
// use std::env;
// use std::sync::Arc;
// use hex;

// use ethers::prelude::{abigen, Address, Http, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, TransactionRequest};


// abigen!(
//     FibonacciVerifier,
//     r#"[function addVerifier(bytes memory sealSelector)]"#,
// );

// #[tokio::main]
fn main() {
    // dotenv().ok();
    // let rpc_url: String = format!(
    //     "https://{}.g.alchemy.com/v2/{}",
    //     env::var("NETWORK").unwrap().as_str(),
    //     env::var("ALCHEMY_API_KEY").unwrap().as_str()
    // );
    // let chain_id = env::var("CHAIN_ID").unwrap().parse::<u64>().unwrap(); 

    // let provider = Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to create provider");
    // let private_key = env::var("PRIVATE_KEY_HOLESKY").expect("PRIVATE_KEY must be set");
    // let wallet: LocalWallet = private_key.parse::<LocalWallet>().expect("invalid private key").with_chain_id(chain_id) ;
    // let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    // // Teacher verifier contract address
    // let fibonachi_verifier_contract_address: Address = ""
    //     .parse()
    //     .unwrap();

    // let fibonachi_verifier_contract = FibonacciVerifier::new(fibonachi_verifier_contract_address, client.clone());

    // let payload_teacher_private_input : PayloadRequest = PayloadRequest {
    //     times : 15,
    //     x : 0,
    //     y : 10
    // }; 
    
    // let receipt = fibo_enhance_proof(payload_teacher_private_input).expect("Failed to generating receipt"); // .expect("Failed to generate proof data");   

    // let journal = receipt.clone().journal.bytes ; 
    // let seal = receipt.clone().claim().unwrap().digest().as_bytes().to_vec();

    // let call_data = fibonachi_verifier_contract.add_verifier(seal.clone().into()) ; 
    // let transaction = TransactionRequest::new()
    //     .to(fibonachi_verifier_contract_address)
    //     .data(call_data.calldata().unwrap())
    //     .gas(300_000u64)
    //     .chain_id(chain_id);

    // let pending_tx = client.send_transaction(transaction, None).await.expect("Failed to send transaction") ;
    // let tx_hash = pending_tx.tx_hash() ; 
    // println!("Transaction hash: {:?}", tx_hash);
}
