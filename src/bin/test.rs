use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use learn::Env;

fn main() {
    let env = Env::new();
    let keypair = Keypair::from_base58_string(&env.private_key_0);
    dbg!(keypair.pubkey());
}
