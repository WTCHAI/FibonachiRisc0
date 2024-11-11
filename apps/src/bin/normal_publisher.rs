use apps::utils::generating_proof::{normal_proof, bonsai_proof} ; 

use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{sha::Digestible};

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio ; 

use ethers::{abi::encode, prelude::{abigen, Address, Http, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, TransactionRequest}};
use ethers::types::U256 ; 

abigen!(
    FibonacciVerifier,
    r#"[function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal)]"#,
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

    // ----------current contract is just based fibonachi contract hardhat/contract/FibonachiVerfier.sol see more details in scripts/deployFibo.ts
    let fibonachi_verifier_contract_address: Address = "0x1E0122e128b316381E439e3aAcDD1aE88E7669F7"
        .parse()
        .unwrap();

    let fibonachi_verifier_contract = FibonacciVerifier::new(fibonachi_verifier_contract_address, client.clone());

    // This normal publisher can generate proof totally fine without bonsai api in .env files 
    // ------------------ On the otherhands having bonsai key in .env cause error ---------- 
    // thread 'main' panicked at /Users/wtchai/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.41.1/src/runtime/blocking/shutdown.rs:51:21:
    // Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    let receipt = normal_proof().expect("Failed to generating receipt");

    // ---------------- using bonsai_proof with apikey in .env -------------- 
    // called `Result::unwrap()` on an `Err` value: server error `{"message":"Forbidden"}`
    // let receipt = bonsai_proof().await.expect("Failed to generating receipt");

    let journal = receipt.clone().journal.bytes ; 
    // seal encode in starks
    let seal_starks = receipt.clone().claim().unwrap().digest().as_bytes().to_vec();
    // seal encode in snarks case for bonsai
    // let seal_snarks = encode_seal(&receipt) ; 

    // The verifier contracts in hardhat/contract/FibonachiVerfier.sol
    // Please know that I've learn zk just a couple weeks.I've heard & start risc0 rust based language from mentor in the invisible garden hackerhouse 
    // Here my question about the process of how we going to verify onchain when it's come to the environtment usecase 
    // I Research a lot about what it should be in the process of verify onchain but still not 100% understood
    // Here's a revert that i'm facing but it might because Starks proof cannot verify onchain https://app.blocksec.com/explorer/tx/holesky/0xc5d130cbf1bd6904b56216ef4021c34e62e05ae305a396c9933eacc5b4795eef?line=2 
    // even if i know that strak cannot proof but I've tried add seal[0:4] selector manually as a admin to Risc0VerfiyRouter but still facing revert.

    // In the case that teachers gave an example for Student finding exact both input x,y for fibonachi where result of fibo equals to public output
    // 1. Teachers have to published the fiboverifier with correct seal & imageId & journalDigest onchain also having methods verify which will call to Risc0VerfiyRouter
    // 2. Teachers deploy Risc0VerfiyRouter and init correct seal[0:4] bytes into verifiers mapping and call again ? Here the part that i still confused.
    // 3. What Teacher have to setup in term of prepare contracts 
    // 4. How the flow of students will called the FibonachiVerfier.verify() ? 
    // 5. Is the process compare the computed proof in the end cause i've try fork testnet to add the seal[0:4] in Risc0VerfiyRouter as a admin but prover still facing revert unknowSelector
    // 6. In my Project for ethglobals use cases have similars flow of these 2 user roles have to proof and verify onchain. 
    // Please know that i've already spend a lot of time in examples repo & risc0ethreum already.

    let call_data = fibonachi_verifier_contract.verify_and_finalize_fibonachi(seal_starks.clone().into(), journal.clone().into()) ; 

    let transaction = TransactionRequest::new()
        .to(fibonachi_verifier_contract_address)
        .data(call_data.calldata().unwrap())
        .gas(300_000u64)
        .chain_id(chain_id);

    let pending_tx = client.send_transaction(transaction, None).await.expect("Failed to send transaction") ;
    let tx_hash = pending_tx.tx_hash() ; 
    println!("Transaction hash: {:?}", tx_hash);
}