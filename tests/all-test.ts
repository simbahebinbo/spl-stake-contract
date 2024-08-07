import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {SplStake} from "../target/types/spl_stake";

const {SystemProgram} = anchor.web3;
const {TOKEN_PROGRAM_ID, Token, createMint} = require('@solana/spl-token');

describe('spl-stake', () => {

    // Configure the client to use the local cluster.
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SplStake as Program<SplStake>;


//     it("Is All", async () => {
//         const accounts = [];
//         const deployer = anchor.web3.Keypair.generate();
//         accounts.push(deployer);
//         const admin = anchor.web3.Keypair.generate();
//         accounts.push(admin);
//         const user = anchor.web3.Keypair.generate();
//         accounts.push(user);
//         const stakingAccount = anchor.web3.Keypair.generate();
//         accounts.push(stakingAccount);
// const stakingTokenAccount = anchor.web3.Keypair.generate();
//         accounts.push(stakingTokenAccount);
//         const userAccount = anchor.web3.Keypair.generate();
//         accounts.push(userAccount);
//
//         let initLamports = 1000000000
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
//         const mintAuthority = anchor.web3.Keypair.generate();
//         const mint = await createMint(
//             provider.connection,
//             admin,
//             mintAuthority.publicKey,
//             null,
//             9, // Decimal places
//         );
//
//         const userTokenAccountAuthority = anchor.web3.Keypair.generate();
//         const userTokenAccount = await getOrCreateAssociatedTokenAccount(
//             provider.connection,
//             admin,
//             mint,
//             userTokenAccountAuthority.publicKey
//         );
//
//
// //         await program.methods
// //                 .initialize(admin.publicKey)
// //                 .accounts({
// //                     stakingAccount: stakingAccount.publicKey,
// //                     deployer: deployer.publicKey,
// //                     // @ts-ignore
// //                     systemProgram: SystemProgram.programId,
// //                 })
// //                 .signers([stakingAccount, deployer])
// //                 .rpc();
// //
// //         const stakingAccountRet1 = await program.account.stakingAccount.fetch(stakingAccount.publicKey);
// //         assert.ok(stakingAccountRet1.admin.equals(admin.publicKey));
// //
// //
// //         const supportedToken = anchor.web3.Keypair.generate().publicKey;
// //
// //         await program.methods
// //             .setSupportedToken(supportedToken)
// //             .accounts({
// //                 stakingAccount: stakingAccount.publicKey,
// //                 admin: admin.publicKey,
// //             })
// //             .signers([admin])
// //             .rpc();
// //
// //         const stakingAccountRet2 = await program.account.stakingAccount.fetch(stakingAccount.publicKey);
// //         assert.ok(stakingAccountRet2.supportedToken.equals(supportedToken));
// //
// //
// //         let mint_amount = 1_000_000_000;
// //
// //         await program.methods
// //             .faucet(new anchor.BN(mint_amount))
// //             .accounts({
// //                 mint: mint,
// //                 userTokenAccount: userTokenAccount.address,
// //                 admin: admin.publicKey,
// //                 // @ts-ignore
// //                 tokenProgram: TOKEN_PROGRAM_ID,
// //             })
// //             .signers([admin])
// //             .rpc();
//
//         await program.methods
//             .faucet(new anchor.BN(1000))
//             .accounts({
//                 mint: mint,
//                 userTokenAccount: userTokenAccount.address,
//                 admin: admin.publicKey,
//                 tokenProgram: TOKEN_PROGRAM_ID,
//             })
//             .signers([admin])
//             .rpc();
//
// //         // Fetch the user token account after calling faucet
// //         const userTokenAccountInfo = await getAccount(provider.connection, userTokenAccount.address);
// //
// //         // Check if the user token account received the correct amount of tokens
// //         assert.ok(userTokenAccountInfo.amount ===mint_amount);
//
//         console.log("Faucet operation successful, user received 1000 tokens");
//
// // const userTokenAccountRet = await program.account.userAccount.fetch(userTokenAccount.publicKey);
// // assert.ok(userTokenAccountRet.balance.eq(new anchor.BN(mint_amount)));
//
//
// //         let deposit_amount = 1000;
// //
// //         await program.methods
// //             .deposit(new anchor.BN(deposit_amount))
// //             .accounts({
// //                 userAccount: userAccount.publicKey,
// //                 user: admin.publicKey,
// //                 userTokenAccount: userTokenAccount.publicKey,
// //                 stakingTokenAccount: stakingTokenAccount.publicKey,
// //                 // @ts-ignore
// //                 tokenProgram: TOKEN_PROGRAM_ID,
// //             })
// //             .signers([admin])
// //             .rpc();
// //
// // const userAccountRet = await program.account.userAccount.fetch(userAccount.publicKey);
// // assert.ok(userAccountRet.balance.eq(new anchor.BN(deposit_amount)));
//
//     });
});


//
// const admin = provider.wallet.publicKey;
// const userTokenAccount = anchor.web3.Keypair.generate();
// const stakingTokenAccount = anchor.web3.Keypair.generate();
//
// await program.rpc.withdraw(new anchor.BN(500), {
//     accounts: {
//         userAccount: userAccount.publicKey,
//         admin: admin,
//         userTokenAccount: userTokenAccount.publicKey,
//         stakingTokenAccount: stakingTokenAccount.publicKey,
//         tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
//     },
//     signers: [],
// });
//
// const account = await program.account.userAccount.fetch(userAccount.publicKey);
// assert.ok(account.balance.eq(new anchor.BN(500)));
//
// const mint = anchor.web3.Keypair.generate();
// const userTokenAccount = anchor.web3.Keypair.generate();
// const admin = provider.wallet.publicKey;
//
// await program.rpc.faucet(new anchor.BN(1000), {
//     accounts: {
//         mint: mint.publicKey,
//         userTokenAccount: userTokenAccount.publicKey,
//         admin: admin,
//         tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
//     },
//     signers: [],
// });
//
// const admin = provider.wallet.publicKey;
//
// await program.rpc.resetUserAccount({
//     accounts: {
//         userAccount: userAccount.publicKey,
//         admin: admin,
//         systemProgram: anchor.web3.SystemProgram.programId,
//     },
//     signers: [],
// });
//
// const account = await program.account.userAccount.fetch(userAccount.publicKey);
// assert.ok(account.balance.eq(new anchor.BN(0)));
//
//
// // Check the balance of the user token account here (assuming you have a way to fetch it)
//
// //     });
// // });
//
//
// it('Simulate', async () => {
//     const payer = provider.wallet.publicKey;
//     const mint = anchor.web3.Keypair.generate();
//     const tokenAccount = anchor.web3.Keypair.generate();
//
//     await program.rpc.simulate({
//         accounts: {
//             payer: payer,
//             tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
//             tokenAccount: tokenAccount.publicKey,
//             mint: mint.publicKey,
//         },
//         signers: [],
//     });
//
//     // Add assertions as necessary
// });
// //


// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);
//
// const program = anchor.workspace.SplStake;
// const user = anchor.web3.Keypair.generate();
// const stakingAccount = anchor.web3.Keypair.generate();
// const userTokenAccount = anchor.web3.Keypair.generate();
// const stakingTokenAccount = anchor.web3.Keypair.generate();
//
// it('Deposit', async () => {
//     // Request airdrop
//     let airdrop_tx = await provider.connection.requestAirdrop(
//         user.publicKey,
//         1000000000
//     );
//     const latestBlockHash = await provider.connection.getLatestBlockhash();
//     await provider.connection.confirmTransaction({
//         blockhash: latestBlockHash.blockhash,
//         lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//         signature: airdrop_tx
//     });
//
//     // Initialize staking account
//     const admin = provider.wallet.publicKey;
//     await program.rpc.initialize(admin, {
//         accounts: {
//             stakingAccount: stakingAccount.publicKey,
//             deployer: provider.wallet.publicKey,
//             systemProgram: anchor.web3.SystemProgram.programId,
//         },
//         signers: [stakingAccount],
//     });
//
//     // Deposit
//     const amount = new anchor.BN(1000);
//     await program.rpc.deposit(amount, {
//         accounts: {
//             userAccount: user.publicKey,
//             user: user.publicKey,
//             userTokenAccount: userTokenAccount.publicKey,
//             stakingTokenAccount: stakingTokenAccount.publicKey,
//             tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
//         },
//         signers: [user],
//     });
//
//     // Fetch and assert the user account balance
//     const userAccount = await program.account.userAccount.fetch(user.publicKey);
//     assert.ok(userAccount.balance.eq(amount));
//
//     // Listen for the DepositEvent
//     const depositEvent = await program.addEventListener('DepositEvent', (event, slot) => {
//         console.log('DepositEvent:', event);
//         assert.ok(event.user.equals(user.publicKey));
//         assert.ok(event.amount.eq(amount));
//     });
//
//     // Remove event listener
//     await program.removeEventListener(depositEvent);
// });
//
// it('Withdraw', async () => {
//     // Withdraw
//     const amount = new anchor.BN(500);
//     await program.rpc.withdraw(amount, {
//         accounts: {
//             userAccount: user.publicKey,
//             admin: provider.wallet.publicKey,
//             userTokenAccount: userTokenAccount.publicKey,
//             stakingTokenAccount: stakingTokenAccount.publicKey,
//             tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
//         },
//         signers: [user],
//     });
//
//     // Fetch and assert the user account balance
//     const userAccount = await program.account.userAccount.fetch(user.publicKey);
//     assert.ok(userAccount.balance.eq(new anchor.BN(500)));
//
//     // Listen for the WithdrawEvent
//     const withdrawEvent = await program.addEventListener('WithdrawEvent', (event, slot) => {
//         console.log('WithdrawEvent:', event);
//         assert.ok(event.user.equals(user.publicKey));
//         assert.ok(event.amount.eq(amount));
//     });
//
//     // Remove event listener
//     await program.removeEventListener(withdrawEvent);
// });
// });

