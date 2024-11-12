use std::result::Result;
use dotenv::dotenv;
use std::env;
use serde::Serialize;

use methods::FINALIZE_FIBONACHI_ELF;

use crate::utils::receipt_interface::PayloadRequest ; 
use crate::utils::receipt_interface::print_receipt_properties ; 
use risc0_zkvm::{ default_prover , ExecutorEnv, Receipt, VerifierContext,ProverOpts};


pub fn normal_proof() -> Result<Receipt, Box<dyn std::error::Error>> {
    #[derive(Serialize)]
    struct Payload{
        times : u64,
        x : u64, 
        y : u64,
    }
    

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    
    let payload = Payload {
        times: 10,
        x: 0,
        y: 10
    };
    
    // Creating env as environtment for our program
    let env = ExecutorEnv::builder()
        .write(&payload)
        .unwrap()
        .build()
        .unwrap();
    
    // Init prove and guest computation
    let prover = default_prover().prove(env, FINALIZE_FIBONACHI_ELF);
    // let prover_ctx = default_prover().prove_with_ctx(env, &VerifierContext::default(), FINALIZE_FIBONACHI_ELF, &ProverOpts::groth16()) ;
    // Receipt is a Proof that guest code computation inside vm known as Proof : ChatGPT

    let receipt: Receipt = prover.unwrap().receipt;
    // let receipt: Receipt = prover_ctx.unwrap().receipt;

    Ok(receipt)
}

use tokio::task;
pub async fn bonsai_proof () -> Result<Receipt, Box<dyn std::error::Error>> {
    dotenv().ok();
    
    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
    .init();

    let payload_internal_env = PayloadRequest {
        times: 10,
        x: 0,
        y: 15,
    };
    
    let receipt = task::spawn_blocking(move || {
        let env = ExecutorEnv::builder()
        .write(&payload_internal_env)
        .unwrap()
        .build()
        .unwrap();

        let prover_ctx = default_prover().prove_with_ctx(
            env,
            &VerifierContext::default(),
            FINALIZE_FIBONACHI_ELF,
            &ProverOpts::groth16()
        );
        prover_ctx.unwrap().receipt
    }).await.unwrap() ; 

    Ok(receipt)
    
}