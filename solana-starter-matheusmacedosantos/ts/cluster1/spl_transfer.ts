import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "../../ts/cluster1/wallet/id.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("Hej4tQLbH9QeLdRkH4VZQJGHAoPjB94QUo7GYWTao9jL");

// Recipient address
const to = new PublicKey("8T9GcCqpjy3NJX1nMkhVPhzJW8zgFJhNxASn4whhRTX2");

(async () => {
  try {
    const ata_from = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const ata_to = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );

    const tx = await transfer(
      connection,
      keypair,
      ata_from.address,
      ata_to.address,
      keypair.publicKey,
      1e6
    );

    console.log(`Your transfer txid: ${tx}`);

    // Get the token account of the fromWallet address, and if it does not exist, create it

    // Get the token account of the toWallet address, and if it does not exist, create it

    // Transfer the new token to the "toTokenAccount" we just created
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
