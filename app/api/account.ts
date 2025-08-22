import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export function getNftMintAccount(id: string) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("MyNFT_v1"),
        Buffer.from(id)
    ], program.programId);
    return mintAccount;
}