mod programs;

#[cfg(test)]

mod tests {
    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_program;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    #[test]
    fn keygen() {
        let kp = Keypair::new();

        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58 (Phantom format):");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Your wallet file (byte array) is:");
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
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

        let base58 = bs58::encode(wallet).into_string();
        println!("Your private key in base58 (Phantom format) is:");
        println!("{:?}", base58);
    }

    const RPC_URL: &str = "https://api.devnet.solana.com";
    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    signature.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("8T9GcCqpjy3NJX1nMkhVPhzJW8zgFJhNxASn4whhRTX2").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)], // 0.1 SOL
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet() {
        // Carrega o keypair do arquivo dev-wallet.json
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Endereço da sua carteira Turbin3
        let to_pubkey = Pubkey::from_str("8T9GcCqpjy3NJX1nMkhVPhzJW8zgFJhNxASn4whhRTX2").unwrap();

        // Conexão com a Solana Devnet
        let rpc_client = RpcClient::new(RPC_URL);

        // Obtem o blockhash mais recente (necessário para assinar a transação)
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // 1. Descobre quanto SOL existe na carteira de origem (em lamports)
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        if balance == 0 {
            println!("A carteira já está vazia.");
            return;
        }

        println!("Saldo atual da carteira: {} lamports", balance);

        // 2. Cria uma mensagem MOCK para calcular a taxa de envio
        let message = Message::new_with_blockhash(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance, // manda tudo (antes de descontar a taxa)
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // 3. Pergunta ao RPC qual seria a taxa para essa transação
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!("Fee calculada: {} lamports", fee);

        // Verificação simples
        if fee > balance {
            println!("Você não tem saldo suficiente para cobrir a taxa!");
            return;
        }

        // 4. Cria a transação final ajustando o valor enviado = balance - fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        // 5. Envia a transação e confirma
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        // Confirma que o saldo agora deve ser 0
        let final_balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get final balance");
        println!("Saldo final da carteira é: {} lamports", final_balance);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"matheusmacedosantos".to_vec(),
        };
        let blockhash = rpc_client.get_latest_blockhash().expect(
            "Failed to get recent
        blockhash",
        );
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:
https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
