import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorSocial } from "../../target/types/anchor_social";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.AnchorSocial as Program<AnchorSocial>;

export { program, provider };


// 默认的本地钱包
export function useDefaultWallet() {
    return anchor.Wallet.local();
}

// 访客钱包 pubkey 2tQibfGH1UX4PEA1dNWJ8zvnpSbKRBr5a5eVhaRxF6EX
export function useVisitorWallet() {
    const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([171, 156, 148, 124, 104, 176, 9, 2, 154, 150, 6, 98, 26, 44, 195, 72, 36, 48, 177, 236, 11, 14, 51, 191, 86, 195, 17, 25, 245, 204, 132, 97, 182, 190, 252, 253, 157, 255, 137, 212, 199, 44, 93, 217, 16, 96, 16, 37, 177, 6, 222, 115, 87, 46, 130, 183, 247, 175, 115, 174, 99, 240, 53, 188]));

    return new anchor.Wallet(keypair);
}
// 35vQtxXXv5rb99eiVrVVrwYMRYc7vscZvXas8zjEnnK5