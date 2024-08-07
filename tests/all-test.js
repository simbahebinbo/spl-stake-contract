// // // const assert = require('assert');
// // // const {SystemProgram} = anchor.web3;
// // //
// // // import * as anchor from "@coral-xyz/anchor";
// // //
// // // describe('spl-stake', () => {
// // //     // Configure the client to use the local cluster.
// // //     let provider = anchor.AnchorProvider.env();
// // //     anchor.setProvider(provider);
// // //
// // //     const program = anchor.workspace.SplStake;
// // //     const user = anchor.web3.Keypair.generate();
// // //     let stakingAccount = anchor.web3.Keypair.generate();
// // //
// // //
// // //     it('Initialize', async () => {
// // //         // Add your test here.
// // //         let airdrop_tx = await provider.connection.requestAirdrop(
// // //             user.publicKey,
// // //             1000000000
// // //         );
// // //         const latestBlockHash = await provider.connection.getLatestBlockhash();
// // //         await provider.connection.confirmTransaction({
// // //             blockhash: latestBlockHash.blockhash,
// // //             lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
// // //             signature: airdrop_tx
// // //         })
// // //
// // //         //     const admin = provider.wallet.publicKey;
// // //         //
// // //         //     await program.rpc.initialize(admin, {
// // //         //         accounts: {
// // //         //             stakingAccount: stakingAccount.publicKey,
// // //         //             deployer: provider.wallet.publicKey,
// // //         //             systemProgram: SystemProgram.programId,
// // //         //         },
// // //         //         signers: [stakingAccount],
// // //         //     });
// // //         //
// // //         //     const account = await program.account.stakingAccount.fetch(stakingAccount.publicKey);
// // //         //     assert.ok(account.admin.equals(admin));
// //
// // const supportedToken = anchor.web3.Keypair.generate().publicKey;
// // const admin = provider.wallet.publicKey;
// //
// // await program.rpc.setSupportedToken(supportedToken, {
// //     accounts: {
// //         stakingAccount: stakingAccount.publicKey,
// //         admin: admin,
// //     },
// // });
// //
// // const account = await program.account.stakingAccount.fetch(stakingAccount.publicKey);
// // assert.ok(account.supportedToken.equals(supportedToken));
// //
// // const user = provider.wallet.publicKey;
// // const userTokenAccount = anchor.web3.Keypair.generate();
// // const stakingTokenAccount = anchor.web3.Keypair.generate();
// //
// // await program.rpc.deposit(new anchor.BN(1000), {
// //     accounts: {
// //         userAccount: userAccount.publicKey,
// //         user: user,
// //         userTokenAccount: userTokenAccount.publicKey,
// //         stakingTokenAccount: stakingTokenAccount.publicKey,
// //         tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
// //     },
// //     signers: [],
// // });
// //
// // const account = await program.account.userAccount.fetch(userAccount.publicKey);
// // assert.ok(account.balance.eq(new anchor.BN(1000)));
// //
// //
// // const admin = provider.wallet.publicKey;
// // const userTokenAccount = anchor.web3.Keypair.generate();
// // const stakingTokenAccount = anchor.web3.Keypair.generate();
// //
// // await program.rpc.withdraw(new anchor.BN(500), {
// //     accounts: {
// //         userAccount: userAccount.publicKey,
// //         admin: admin,
// //         userTokenAccount: userTokenAccount.publicKey,
// //         stakingTokenAccount: stakingTokenAccount.publicKey,
// //         tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
// //     },
// //     signers: [],
// // });
// //
// // const account = await program.account.userAccount.fetch(userAccount.publicKey);
// // assert.ok(account.balance.eq(new anchor.BN(500)));
// //
// // const mint = anchor.web3.Keypair.generate();
// // const userTokenAccount = anchor.web3.Keypair.generate();
// // const admin = provider.wallet.publicKey;
// //
// // await program.rpc.faucet(new anchor.BN(1000), {
// //     accounts: {
// //         mint: mint.publicKey,
// //         userTokenAccount: userTokenAccount.publicKey,
// //         admin: admin,
// //         tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
// //     },
// //     signers: [],
// // });
// //
// // const admin = provider.wallet.publicKey;
// //
// // await program.rpc.resetUserAccount({
// //     accounts: {
// //         userAccount: userAccount.publicKey,
// //         admin: admin,
// //         systemProgram: anchor.web3.SystemProgram.programId,
// //     },
// //     signers: [],
// // });
// //
// // const account = await program.account.userAccount.fetch(userAccount.publicKey);
// // assert.ok(account.balance.eq(new anchor.BN(0)));
// //
// //
// // // Check the balance of the user token account here (assuming you have a way to fetch it)
// //
// // //     });
// // // });
// //
// //
// // it('Simulate', async () => {
// //     const payer = provider.wallet.publicKey;
// //     const mint = anchor.web3.Keypair.generate();
// //     const tokenAccount = anchor.web3.Keypair.generate();
// //
// //     await program.rpc.simulate({
// //         accounts: {
// //             payer: payer,
// //             tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
// //             tokenAccount: tokenAccount.publicKey,
// //             mint: mint.publicKey,
// //         },
// //         signers: [],
// //     });
// //
// //     // Add assertions as necessary
// // });
// // //
//
//
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