use risc0_zkvm::{ default_prover , ExecutorEnv, Receipt, VerifierContext,ProverOpts};

use std::result::Result;
use dotenv::dotenv;

use methods::FINALIZE_FIBONACHI_ELF;

use crate::utils::receipt_interface::{PayloadRequest, FrontPayloadRequest} ; 
use alloy_primitives::U256;
use alloy_sol_types::SolValue;

use serde::Serialize;

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
    
    // Assume this PayloadRequest is receiving integers from the frontend
    let frontend_payload = FrontPayloadRequest {
        times: 10, // Received as integer
        x: 0,
        y: 15,
    };

    // Dynamically convert frontend integers to U256
    let payload = PayloadRequest {
        times: U256::from(frontend_payload.times),
        x: U256::from(frontend_payload.x),
        y: U256::from(frontend_payload.y),
    };

    let payload_json = serde_json::to_vec(&payload).unwrap(); 
    
    let receipt = task::spawn_blocking(move || {
        let env = ExecutorEnv::builder()
        .write(&payload_json)
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