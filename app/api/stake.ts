import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";
import { getNftMintAccount } from "./account";

export async function stakeNFT(wallet: anchor.Wallet, nft_id: string) {
    return await program.methods.nftStake().accounts({
        nftMintAccount: getNftMintAccount(nft_id),
    }).signers([wallet.payer]).rpc();
}