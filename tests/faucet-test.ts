// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SplStake } from "../target/types/spl_stake";
// import { assert } from "chai";
// import { getAccount, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
// const { TOKEN_PROGRAM_ID, createMint } = require('@solana/spl-token');
//
// describe('spl-stake', () => {
//     // Configure the client to use the local cluster.
//     let provider = anchor.AnchorProvider.env();
//     anchor.setProvider(provider);
//
//     const program = anchor.workspace.SplStake as Program<SplStake>;
//
//     it("Is Faucet", async () => {
//         const accounts = [];
//         const admin = anchor.web3.Keypair.generate();
//         accounts.push(admin);
//
//         let initLamports = 1000000000;
//
//         // 发起多个空投请求
//         const airdropPromises = accounts.map(async (account) => {
//             const airdrop_tx = await provider.connection.requestAirdrop(account.publicKey, initLamports);
//             return { airdrop_tx, account };
//         });
//
//         // 等待所有空投请求完成
//         const airdropResults = await Promise.all(airdropPromises);
//
//         // 获取最新的区块哈希
//         const latestBlockHash = await provider.connection.getLatestBlockhash();
//
//         // 确认每个空投交易
//         const confirmationPromises = airdropResults.map(({ airdrop_tx, account }) =>
//             provider.connection.confirmTransaction({
//                 blockhash: latestBlockHash.blockhash,
//                 lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//                 signature: airdrop_tx
//             }).then((confirmation) => {
//                 console.log(`Airdrop to ${account.publicKey.toBase58()} confirmed`, confirmation);
//                 return confirmation;
//             }).catch((error) => {
//                 console.error(`Error confirming airdrop to ${account.publicKey.toBase58()}`, error);
//                 throw error;
//             })
//         );
//
//         // 等待所有确认请求完成
//         await Promise.all(confirmationPromises);
//
//         console.log("All airdrops confirmed.");
//
//         console.log("TOKEN_PROGRAM_ID: ", TOKEN_PROGRAM_ID.toString());
//
//         const mintAuthority = anchor.web3.Keypair.generate();
//         const mint = await createMint(
//             provider.connection,
//             admin,
//             mintAuthority.publicKey,
//             null,
//             9, // Decimal places
//             // TOKEN_PROGRAM_ID,
//         );
//         console.log("Mint created: ", mint.toBase58());
//
//
//
//         const userTokenAccountAuthority = anchor.web3.Keypair.generate();
//         const userTokenAccount = await getOrCreateAssociatedTokenAccount(
//             provider.connection,
//             admin,
//             userTokenAccountAuthority.publicKey,
//             mint,
//             // TOKEN_PROGRAM_ID,
//         );
//         console.log("Associated Token Account: ", userTokenAccount.address.toBase58());
//
//         let mint_amount = 1_000_000_000;
//
//         await program.methods
//             .faucet(new anchor.BN(mint_amount))
//             .accounts({
//                 mint: mint,
//                 userTokenAccount: userTokenAccount.address,
//                 admin: admin.publicKey,
//                 // @ts-ignore
//                 tokenProgram: TOKEN_PROGRAM_ID,
//             })
//             .signers([admin])
//             .rpc();
//
//         const userTokenAccountInfo = await getAccount(provider.connection, userTokenAccount.address);
//         assert.ok(userTokenAccountInfo.amount === BigInt(mint_amount));
//
//         console.log("Faucet operation successful, user received tokens", mint_amount);
//     });
// });
