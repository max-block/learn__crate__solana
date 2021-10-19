use std::{error::Error, str::FromStr};

use solana_sdk::pubkey::Pubkey;

fn main() -> Result<(), Box<dyn Error>> {
    println!("it works");
    let program_id = Pubkey::from_str("D8mKxaBrjSBDDB5mXBWvrVeT4Y8YsuFAjLnFBXEGueVP")?;
    let seed1 = "seed111";
    let seed2 = "seed22";

    let pda1 = Pubkey::create_program_address(&[seed1.as_bytes()], &program_id)?;
    println!("pda1: {}", pda1);
    // pda1: 3E3HXDXDQMaLgYNHy4rmPZZ9AReShZW2oybgVmSCR7GD

    match Pubkey::create_program_address(&[seed2.as_bytes()], &program_id) {
        Ok(pda2) => println!("pda2: {}", pda2),
        Err(err) => {
            println!("pda2 error: {}", err);
        }
    };
    // pda2 error: Provided seeds do not result in a valid address

    Ok(())
}
