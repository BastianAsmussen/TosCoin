use anyhow::Result;
use tos_coin::coin::chains::CHAIN;
use tos_coin::coin::wallets::Wallet;

#[allow(clippy::unwrap_used)]
fn main() -> Result<()> {
    let mut bastian = Wallet::default();
    let mut tosal = Wallet::default();
    let mut malthe = Wallet::default();

    let solution = bastian.send(50.0, tosal.public_key)?;
    println!("Sent 50.0 from Bastian to Tosal. (Solution: {solution:?})");

    let solution = tosal.send(25.0, malthe.public_key)?;
    println!("Sent 25.0 from Tosal to Malthe. (Solution: {solution:?})");

    let solution = malthe.send(5.0, bastian.public_key)?;
    println!("Sent 5.0 from Malthe to Bastian. (Solution: {solution:?})");

    let json = serde_json::to_string_pretty(&CHAIN.lock().unwrap().blocks)?;
    println!("Chain: {json}");

    Ok(())
}
