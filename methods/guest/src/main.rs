use risc0_zkvm::guest::env;

use serde::Deserialize;  // Import Deserialize to read structs
#[derive(Deserialize)]
// Declare struct to use as input of guest program from env which set at host side 
struct Payload {
    times: u32 ,
    x: u64,
    y: u64,
}

fn main() {
    // Reading Payload data from host through env which create by host
    let payload : Payload = env::read() ;     
    
    // Declare loop times which is 100 times

    // println!("Guest side : {}", n) ;

    // getting result from fibonachi process 
    let result : u64 = fibonachi_logic(payload.times, payload.x, payload.y) ;
    // Commiting result specific into env called journal
    env::commit(&result);
}

// We are creating a fibonachi process getting 
// x and y as starting value of fibonachi limited with n fibo is 100
// we wanted to prove that any starting values can be fibonachi too maximum fibo times
// this is the way we wanted to prove we know fibonachi
fn fibonachi_logic(t : u32, mut  x : u64, mut  y : u64) -> u64  {
    if t == 0  { 
        return x ; 
    }else if  t == 1  { 
        return y ; 
    }
    // Consider doing loop way to find fibonachi 
    else { 
        for _ in 2..=t { 
            let z: u64  = x + y ; 
            x = y ;
            y = z  ;
        }
        return y  ; 
    }
    // OR recursively
    // fibonachi_logic(n-1 , y, y+x )
}