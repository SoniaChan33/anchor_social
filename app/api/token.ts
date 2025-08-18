import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTokenMintAccount(wallet: anchor.Wallet) {
    const [splTokenPda,] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint"),],
        program.programId,
    );

    return [splTokenPda, program.methods.createTokenMintAccount().accounts({

    }).rpc()];
}