// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_program;
// use anchor_spl::token::{self, Mint, Token, TokenAccount};
//
// #[tokio::test]
// async fn test_simulate() {
//     let program_id = Pubkey::new_unique();
//     let payer = Pubkey::new_unique();
//     let mint = Pubkey::new_unique();
//     let token_account = Pubkey::new_unique();
//
//     // 初始化上下文
//     let mut program = ProgramTest::new(
//         "my_program",
//         program_id,
//         processor!(my_program::my_instruction),
//     );
//
//     program.add_account(
//         mint,
//         Account {
//             lamports: 1000000,
//             data: vec![0; Mint::LEN],
//             owner: token::ID,
//             executable: false,
//             rent_epoch: 0,
//         },
//     );
//
//     program.add_account(
//         token_account,
//         Account {
//             lamports: 1000000,
//             data: vec![0; TokenAccount::LEN],
//             owner: token::ID,
//             executable: false,
//             rent_epoch: 0,
//         },
//     );
//
//     let (mut banks_client, payer, recent_blockhash) = program.start().await;
//
//     let tx = Transaction::new_signed_with_payer(
//         &[instruction::my_instruction(
//             program_id,
//             payer.pubkey(),
//             token::ID,
//             mint,
//             token_account,
//         )],
//         Some(&payer.pubkey()),
//         &[&payer],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(tx).await.unwrap();
// }
//
