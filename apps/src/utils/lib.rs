use anyhow::{bail, Result};
use risc0_zkvm::{sha::Digestible, InnerReceipt, Receipt};
// use risc0_ethereum_contracts::encode_seal; 

// pub fn encode_seal_lib(receipt: &Receipt) -> Result<Vec<u8>> {
//     // We use Fake here as a stand-in, similar to what would happen without Groth16 on macOS.
//     let seal = match receipt.inner.clone() {
//         InnerReceipt::Fake(fake_receipt) => {
//             let claim_seal = fake_receipt.claim.digest().as_bytes().to_vec();
//             let selector = &[0u8; 4]; // Placeholder selector for testing
//             let mut selector_seal = Vec::with_capacity(selector.len() + claim_seal.len());
//             selector_seal.extend_from_slice(selector);
//             selector_seal.extend_from_slice(&claim_seal);
//             selector_seal
//         }
//         _ => bail!("Unsupported receipt type for mock"),
//     };
//     Ok(seal)
// }


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

    // let seal_encoded_lib = encode_seal(receipt).unwrap() ;
    // println!("Seal encoded lib : {:?}",seal_encoded_lib) ;

}