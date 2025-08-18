import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTweet(
    wallet: anchor.Wallet,
    body: string,
): Promise<[anchor.web3.PublicKey, string]> {
    const [profilePda,] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), wallet.publicKey.toBuffer()],
        program.programId,
    );

    const profile = await program.account.profile.fetch(profilePda);

    const [tweetPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("tweet"),
            profilePda.toBuffer(),
            // TODO 这里的语法不太懂
            Buffer.from((profile.tweetCount + 1).toString()) // 修复：使用 toString() 而不是模板字符串
        ],
        program.programId,

    );


    return [
        tweetPda,
        await program.methods.createTweet(body)
            .accounts({
                authority: wallet.publicKey,
                tweet: tweetPda,
            })
            .rpc()
    ];

}



export async function getTweet(wallet: anchor.Wallet, tweetPda: anchor.web3.PublicKey) {
    return await program.account.tweet.fetch(tweetPda);
}