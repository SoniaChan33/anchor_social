import { useDefaultWallet, useVisitorWallet } from "./api/wallet";
import { createProfile, getProfile } from "./api/profile";
import { createTweet, getTweet } from "./api/tweet";
import { createLike } from "./api/tweet";
import { createTokenMintAccount } from "./api/token";

(async () => {
    const defaultWallet = useDefaultWallet();
    const visitorWallet = useVisitorWallet();

    // // 尝试创建用户资料
    // try {
    //     const r1 = await createProfile(defaultWallet, "Bob");
    //     console.log("Profile created:", r1);
    // } catch (error) {
    //     console.log("Profile might already exist or creation failed:", error.message);
    // }

    // try {
    //     const r2 = await createProfile(visitorWallet, "Alice");
    //     console.log("Visitor profile created:", r2);
    // } catch (error) {
    //     console.log("Visitor profile might already exist or creation failed:", error.message);
    // }

    // // 获取用户资料
    // try {
    //     const profile = await getProfile(defaultWallet);
    //     console.log("Profile fetched:", profile);
    // } catch (error) {
    //     console.log("Failed to fetch profile:", error.message);
    // }

    // try {
    //     const visitorProfile = await getProfile(visitorWallet);
    //     console.log("Visitor profile fetched:", visitorProfile);
    // } catch (error) {
    //     console.log("Failed to fetch visitor profile:", error.message);
    // }

    // // 创建推文
    // // TODO 这里返回的pda和wallet有啥关系
    // const [pda, r3] = await createTweet(defaultWallet, "Hello, world!");

    // const r4 = await getTweet(defaultWallet, pda);
    // console.log(r4);

    // // 创建点赞
    // const r5 = await createLike(visitorWallet, pda);
    // console.log("Like created:", r5);

    // const r6 = await getTweet(defaultWallet, pda);
    // console.log(r6);

    const [tokenPda, r] = await createTokenMintAccount(defaultWallet);
    console.log(tokenPda.toString(), r);

})(); 