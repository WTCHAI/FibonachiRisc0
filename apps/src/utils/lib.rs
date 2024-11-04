use anyhow::{bail, Result};
use risc0_zkvm::{sha::Digestible, Receipt};
use risc0_ethereum_contracts::encode_seal;

// use methods::{
//     FIBONACCI_GUEST_ELF, FIBONACCI_GUEST_ID
// };

pub fn convertImageToU8(vector32: [u32;8]) -> [u8;32] {
    let mut result : [u8;32] = [0u8;32] ; 

    for (i, &value) in vector32.iter().enumerate() {
        let bytes = value.to_le_bytes(); // Convert each u32 to 4 bytes
        let start = i * 4;
        result[start..start + 4].copy_from_slice(&bytes);
    }
    result
}

// pub fn encode_seal_lib(receipt: &risc0_zkvm::Receipt) -> Result<Vec<u8>> {
//     println!("Receipt {:?}",receipt) ;
//     let seal = match receipt.inner.clone() {
//         InnerReceipt::Fake(receipt) => {
//             let seal = receipt.claim.digest().as_bytes().to_vec();
//             let selector = &[0u8; 4];
//             // Create a new vector with the capacity to hold both selector and seal
//             let mut selector_seal = Vec::with_capacity(selector.len() + seal.len());
//             selector_seal.extend_from_slice(selector);
//             selector_seal.extend_from_slice(&seal);
//             selector_seal
//         }
//         InnerReceipt::Groth16(receipt) => {
//             let selector = &receipt.verifier_parameters.as_bytes()[..4];
//             // Create a new vector with the capacity to hold both selector and seal
//             let mut selector_seal = Vec::with_capacity(selector.len() + receipt.seal.len());
//             selector_seal.extend_from_slice(selector);
//             selector_seal.extend_from_slice(receipt.seal.as_ref());
//             selector_seal
//         }
//         _ => bail!("Unsupported receipt type"),
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