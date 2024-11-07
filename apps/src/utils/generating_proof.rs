use std::result::Result;
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::FINALIZE_FIBONACHI_ELF;

use crate::utils::receipt_interface::PayloadRequest ; 

use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

use rand::rngs::OsRng; // Cryptographically secure RNG from the OS
use rand::Rng; // Trait to generate random numbers

pub fn normal_proof() -> Result<Receipt, Box<dyn std::error::Error>> {
    use serde::Serialize;
    // Declare struct payload & serde used for derive struct in rust guess ?
    #[derive(Serialize)]
    struct Payload {
        times: u32,
        x: u64,
        y: u64,
        correct_y: u64,
        binding_randomness: u64,
    }
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    // donno this
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // This is private input knowing only real prover
    let private_inputy = 100;

    // randomness setup
    let mut rng = OsRng;
    let mut binding_randomness_values = rng.gen();

    while binding_randomness_values < private_inputy || binding_randomness_values == 0 {
        println!(
            "Genreating random but got {} rerandom ",
            binding_randomness_values
        );
        binding_randomness_values = rng.gen();
    }

    // Define variable for our program
    let payload = Payload {
        times: 300,
        x: 50,
        y: 100, // Testing random input y to generate proof
        correct_y: private_inputy,
        binding_randomness: binding_randomness_values,
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

pub fn fibo_enhance_proof(payload : PayloadRequest)-> Result<Receipt, Box<dyn std::error::Error>>{
    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
    .init();
    let payload_internal_env : PayloadRequest = PayloadRequest { 
        time : payload.time,
        x : payload.x,
        y : payload.y
    } ; 
    let env = ExecutorEnv::builder()
    .write(&payload_internal_env)
    .unwrap()
    .build()
    .unwrap();

    let prover = default_prover().prove(env, FINALIZE_FIBONACHI_ELF);        
    let receipt: Receipt = prover.unwrap().receipt;
    Ok(receipt)
}
