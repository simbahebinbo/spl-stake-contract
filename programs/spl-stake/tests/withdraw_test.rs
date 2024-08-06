// #[tokio::test]
// async fn test_withdraw() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         admin,
//         staking_account,
//     } = SetUpTest::new();
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     // 假设用户账户是由 signer 生成的
//     let user = Keypair::new();
//     let user_token_account = Keypair::new();
//     let staking_token_account = Keypair::new();
//
//     // 创建并资助用户和质押账户
//     pt.add_account(
//         user.pubkey(),
//         solana_sdk::account::Account {
//             lamports: 1_000_000_000,
//             ..solana_sdk::account::Account::default()
//         },
//     );
//
//     pt.add_account(
//         user_token_account.pubkey(),
//         solana_sdk::account::Account {
//             lamports: 1_000_000_000,
//             ..solana_sdk::account::Account::default()
//         },
//     );
//
//     pt.add_account(
//         staking_token_account.pubkey(),
//         solana_sdk::account::Account {
//             lamports: 1_000_000_000,
//             ..solana_sdk::account::Account::default()
//         },
//     );
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     // 先进行存款操作
//     let deposit_amount: u64 = 500_000_000;
//
//     let deposit_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::Deposit {
//             user_account: user.pubkey(),
//             user: user.pubkey(),
//             user_token_account: user_token_account.pubkey(),
//             staking_token_account: staking_token_account.pubkey(),
//             token_program: spl_token::ID,
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::Deposit { amount: deposit_amount }.data(),
//     };
//
//     let signed_txs = Transaction::new_signed_with_payer(
//         &[deposit_ix],
//         Some(&user.pubkey()),
//         &[&user],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(signed_txs).await.unwrap();
//
//     // 提现操作
//     let withdraw_amount: u64 = 200_000_000;
//
//     let withdraw_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::Withdraw {
//             user_account: user.pubkey(),
//             admin: admin.pubkey(),
//             user_token_account: user_token_account.pubkey(),
//             staking_token_account: staking_token_account.pubkey(),
//             token_program: spl_token::ID,
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::Withdraw { amount: withdraw_amount }.data(),
//     };
//
//     let signed_txs = Transaction::new_signed_with_payer(
//         &[withdraw_ix],
//         Some(&admin.pubkey()),
//         &[&admin],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(signed_txs).await.unwrap();
//
//     // 反序列化并检查用户账户
//     let user_account_data: UserAccount = load_and_deserialize(
//         banks_client,
//         user.pubkey(),
//     ).await;
//
//     // 确保用户账户的余额已更新
//     assert_eq!(user_account_data.balance, deposit_amount - withdraw_amount);
// }
