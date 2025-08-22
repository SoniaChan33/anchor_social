import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTokenMintAccount(wallet: anchor.Wallet) {
    const [splTokenPda,] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint_v3"),],
        program.programId,
    );

    return [splTokenPda,
        await program.methods.createTokenMintAccount().accounts({

        }).rpc()];
}

export async function createTokenMint() {
    return await program.methods.createTokenMintAccount().rpc();
}