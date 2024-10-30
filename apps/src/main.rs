mod utils{ 
    pub mod generating_receipt;
}

use utils::generating_receipt::{generating_receipt} ;

use risc0_zkvm::{ Receipt};

fn main(){
    // wanted to publish generated receipt to sepolia
    let receipt : Receipt = generating_receipt().unwrap() ;
    println!("Receipt : {:?}",receipt) ; 
}