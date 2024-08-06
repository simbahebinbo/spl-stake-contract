// use solana_program::program_pack::Pack;
// use solana_program_test::*;
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     transaction::Transaction,
// };
// use spl_token::{
//     instruction::{initialize_account, initialize_mint, mint_to},
//     state::Mint,
// };
//
// #[tokio::test]
// async fn test_spl_token() {
//     // 创建一个 Solana 测试环境
//     let mut program_test = ProgramTest::new(
//         "spl_token",
//         spl_token::id(),
//         processor!(spl_token::processor::Processor::process),
//     );
//
//     let mint = Keypair::new();
//     let token_account = Keypair::new();
//     let payer = Keypair::new();
//
//     program_test.add_account(
//         mint.pubkey(),
//         solana_sdk::account::Account::new(1_000_000_000, Mint::LEN, &spl_token::id()),
//     );
//     program_test.add_account(
//         token_account.pubkey(),
//         solana_sdk::account::Account::new(1_000_000_000, 0, &spl_token::id()),
//     );
//
//     let mut banks_client = program_test.start_with_context().await.banks_client;
//
//     // 初始化 Mint
//     let mint_ix = initialize_mint(
//         &spl_token::id(),
//         &mint.pubkey(),
//         &payer.pubkey(),
//         Some(&payer.pubkey()),
//         9,
//     )
//         .unwrap();
//
//     let tx = Transaction::new_signed_with_payer(
//         &[mint_ix],
//         Some(&payer.pubkey()),
//         &[&payer, &mint],
//         banks_client.get_latest_blockhash().await.unwrap(),
//     );
//
//     banks_client.process_transaction(tx).await.unwrap();
//
//     // 初始化 Token Account
//     let account_ix = initialize_account(
//         &spl_token::id(),
//         &token_account.pubkey(),
//         &mint.pubkey(),
//         &payer.pubkey(),
//     )
//         .unwrap();
//
//     let tx = Transaction::new_signed_with_payer(
//         &[account_ix],
//         Some(&payer.pubkey()),
//         &[&payer, &token_account],
//         banks_client.get_latest_blockhash().await.unwrap(),
//     );
//
//     banks_client.process_transaction(tx).await.unwrap();
//
//     // 铸造代币
//     let mint_to_ix = mint_to(
//         &spl_token::id(),
//         &mint.pubkey(),
//         &token_account.pubkey(),
//         &payer.pubkey(),
//         &[],
//         1_000_000,
//     )
//         .unwrap();
//
//     let tx = Transaction::new_signed_with_payer(
//         &[mint_to_ix],
//         Some(&payer.pubkey()),
//         &[&payer],
//         banks_client.get_latest_blockhash().await.unwrap(),
//     );
//
//     banks_client.process_transaction(tx).await.unwrap();
//
//     // 验证代币余额
//     let account = banks_client
//         .get_account(token_account.pubkey())
//         .await
//         .unwrap()
//         .unwrap();
//
//     let account_data = spl_token::state::Account::unpack(&account.data).unwrap();
//     assert_eq!(account_data.amount, 1_000_000);
// }
