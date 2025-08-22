import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function nftMint(wallet: anchor.Wallet, nftId: string) {
    const tx = await program.methods.nftMint(nftId).accounts({
        authority: wallet.publicKey,
    }).rpc();
    return tx;
}

export async function nftMintV2(wallet: anchor.Wallet, nftId: string) {
    const tx = await program.methods.nftMint(nftId).accounts({}).signers([wallet.payer]).rpc();
    return tx;
}