// import * as anchor from "@coral-xyz/anchor";
// import {Program} from "@coral-xyz/anchor";
// // @ts-ignore
// import {SplStake} from "../target/types/spl_stake";
//
// describe("spl-stake", () => {
//     // Configure the client to use the local cluster.
//     let provider = anchor.AnchorProvider.env();
//     anchor.setProvider(provider);
//
//     const program = anchor.workspace.SplStake as Program<SplStake>;
//     const user = anchor.web3.Keypair.generate();
//
//     it("Is initialized!", async () => {
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
//         // PDA
//         const [user_data_pda, _] =
//             anchor.web3.PublicKey.findProgramAddressSync(
//                 [anchor.utils.bytes.utf8.encode("hello"), user.publicKey.toBuffer()],
//                 program.programId
//             );
//
//         try {
//             const tx = await program.methods
//                 .initialize({
//                     name: "Ben",
//                     age: 10,
//                 })
//                 .accounts({
//                     // @ts-ignore
//                     data: user_data_pda,
//                     authority: user.publicKey,
//                     systemProgram: anchor.web3.SystemProgram.programId,
//                 })
//                 .signers([user])
//                 .rpc();
//             console.log("Your transaction signature", tx);
//         } catch (error) {
//             console.log(error)
//         }
//     });
// });
//
