use std::{error::Error, str::FromStr};

use solana_sdk::pubkey::Pubkey;

fn pda_with_base() -> Result<(), Box<dyn Error>> {
    let program_id = Pubkey::from_str("D8mKxaBrjSBDDB5mXBWvrVeT4Y8YsuFAjLnFBXEGueVP")?;
    let base_pubkey = Pubkey::from_str("384t4GTu6yzZYcyGQNjYhDw2RNx9jMLwWh3J9HgaJKAk")?;
    let seed1 = "seed111";

    let pda = Pubkey::create_with_seed(&base_pubkey, seed1, &&program_id)?;
    println!("pda: {}", pda);
    // pda: txQK9e43qwDks6NRDWC3hF7CjgZA7Go3J6EYmUT2zwu

    Ok(())
}

fn pda_without_base() -> Result<(), Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    pda_with_base()?;
    pda_without_base()?;
    Ok(())
}
