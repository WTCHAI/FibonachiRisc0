use std::result::Result;
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    FIBONACCI_GUEST_ELF, FIBONACCI_GUEST_ID
};

use risc0_zkvm::{default_prover, ExecutorEnv , Receipt , VerifierContext , ProverOpts } ;

use rand::rngs::OsRng;  // Cryptographically secure RNG from the OS
use rand::Rng;  // Trait to generate random numbers

// Declare struct payload & serde used for derive struct in rust guess ? 
use serde::Serialize;
#[derive(Serialize)] 
struct Payload{
    times : u32,
    x : u64, 
    y : u64,
    correct_y : u64,
    binding_randomness : u64,
}

pub fn generating_proof() -> Result<Receipt, Box<dyn std::error::Error>> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    // donno this 
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // This is private input knowing only real prover 
    let private_inputy = 10 ; 

    // randomness setup
    let mut rng = OsRng ;
    let mut binding_randomness_values = rng.gen() ; 
    
    while binding_randomness_values < private_inputy || binding_randomness_values == 0 {
        println!("Genreating random but got {} rerandom ",binding_randomness_values) ; 
        binding_randomness_values = rng.gen() ;
    }

    // Define variable for our program
    let payload= Payload { 
        times : 15 , 
        x : 0, 
        y : 10,         // Testing random input y to generate proof
        correct_y : private_inputy,
        binding_randomness : binding_randomness_values,
    }; 
    
    // Creating env as environtment for our program 
    let env = ExecutorEnv::builder().write(&payload).unwrap().build().unwrap() ; 
    
    // Init prove and guest computation 
    let prover = default_prover().prove(env, FIBONACCI_GUEST_ELF); 
    // let prover_ctx = default_prover().prove_with_ctx(env, &VerifierContext::default(), FIBONACCI_GUEST_ELF, &ProverOpts::default()) ;
    // Receipt is a Proof that guest code computation inside vm known as Proof : ChatGPT
    let receipt : Receipt = prover.unwrap().receipt ; 
    println!("Receipt : {:?}",receipt) ;
    Ok(receipt)
}
