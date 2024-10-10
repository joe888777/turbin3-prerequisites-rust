use solana_client::rpc_client::RpcClient; use solana_program::{pubkey::Pubkey,system_instruction::transfer,};
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction
};
use std::str::FromStr;
use std::io::{self, BufRead};
use bs58;
use solana_program::system_program;
const RPC_URL: &str = "https://api.devnet.solana.com";
//Import our keypair

#[cfg(test)]
mod programs;
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};
mod tests {
    use super::*;
    #[test]
    fn submit(){
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq",
        signer.pubkey().to_bytes().as_ref()]);
        //Define our instruction data
        let args = CompleteArgs {
        github: b"joe888777".to_vec()
        };
        //Get recent blockhash
        let blockhash = rpc_client 
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        //Now we can invoke the "complete function"
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()], &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );
        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        //Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
    // create a new keypair
    #[test]
    fn keygen(){
        let kp = Keypair::new();
        println!("Tour've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes())
    }
    #[test]
    fn base58_to_wallet(){
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
    #[test]
    fn wallet_to_base58(){
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
    #[test]
    fn airdrop(){
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64){
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }, Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };
    }
    #[test]
    fn transfer_sol(){
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        //Define your Turbin3 public key
        let to_pubkey = Pubkey::from_str("F8XJLRvgPt7WcRQdqEYx2Y9KPdoPRyjTMR5bgfdpotT7").unwrap();
        //Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        //Create a test transaction to calculate fees
        

        //Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let message = Message::new_with_blockhash(
                &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );
        //Calculate exact fe rate to transfer entire SOL amount out of account minus fees
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey, balance - fee
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );
        //Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        //Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
    
}
