
use ethers::types::U256;
use risc0_zkvm::{sha::Digestible , Receipt};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadRequest {
    pub times : u64,
    pub x: u64,
    pub y: u64,
}

pub struct PayloadResponse { 
    pub journal: Vec<u8>,           // Journal data in bytes        // Journal data in hex format
    pub seal: Vec<u8>              // Seal data in bytes
}

pub fn print_receipt_properties(receipt: &Receipt) {
    // Print the full receipt for debugging
    // println!("Receipt claim : {:?}", receipt.claim());
    println!("Receipt inner then claim : {:?}",receipt.inner.claim()) ;
    
    let public_output : u64 = receipt.journal.decode().unwrap() ; 
    println!("Public output : {:?}",public_output) ;

    // Digest is using shar256 hash 
    println!("Receipt : journal : {:?}",receipt.journal.bytes.clone()) ;
    println!("Receipt : journal digest as byte : {:?}",receipt.journal.bytes.clone().digest().as_bytes() ) ;
    println!("Receipt : metadata : {:?}",receipt.metadata.clone().verifier_parameters.as_bytes()) ;

    let seal = receipt.claim().unwrap().digest() ; 
    println!("Seal : {:?}",seal) ;
    println!("Seal as bytes : {:?}",seal.as_bytes()) ;
    println!("Seal as bytes to vec : {:?}",seal.as_bytes().to_vec()) ;
    println!("Seal as bytes to vec : {:?}",seal.as_bytes().to_vec().len()) ;
    let journal = receipt.journal.bytes.clone() ; 
    let public_output = U256::from_big_endian(&journal) ; //receipt.journal.decode().unwrap() ; 
    // Journal maximum uint64 cause computed bytes 8
    println!("public output after decode  : {:?}",public_output) ; 
    println!("Journal : {:?}",journal) ; 
    println!("Seal : {:?}", (seal)) ;
    println!("Journal (Hex): 0x{}", hex::encode(&journal));
    println!("Seal (Hex): 0x{}", hex::encode(&seal));

}

