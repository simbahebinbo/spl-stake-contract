// const assert = require('assert');
// const {SystemProgram} = anchor.web3;
//
// import * as anchor from "@coral-xyz/anchor";
//
// describe('spl-stake', () => {
//     // Configure the client to use the local cluster.
//     let provider = anchor.AnchorProvider.env();
//     anchor.setProvider(provider);
//
//     const program = anchor.workspace.SplStake;
//     const user = anchor.web3.Keypair.generate();
//     let stakingAccount = anchor.web3.Keypair.generate();
//
//
//     it('Initialize', async () => {
//         // Add your test here.
//         let airdrop_tx = await provider.connection.requestAirdrop(
//             user.publicKey,
//             1000000000
//         );
//         const latestBlockHash = await provider.connection.getLatestBlockhash();
//         await provider.connection.confirmTransaction({
//             blockhash: latestBlockHash.blockhash,
//             lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//             signature: airdrop_tx
//         })
//
//         //     const admin = provider.wallet.publicKey;
//         //
//         //     await program.rpc.initialize(admin, {
//         //         accounts: {
//         //             stakingAccount: stakingAccount.publicKey,
//         //             deployer: provider.wallet.publicKey,
//         //             systemProgram: SystemProgram.programId,
//         //         },
//         //         signers: [stakingAccount],
//         //     });
//         //
//         //     const account = await program.account.stakingAccount.fetch(stakingAccount.publicKey);
//         //     assert.ok(account.admin.equals(admin));
//     });
// });
//
