// use anchor_lang::prelude::*;
// use anchor_spl::token;
// use solana_program::instruction::Instruction;
// use solana_program_test::*;
// use solana_sdk::{
//     signature::Signer,
//     transaction::Transaction,
// };
// use solana_sdk::signature::Keypair;
//
// use spl_stake::{self};
//
// #[tokio::test]
// async fn test_faucet() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         admin,
//         mint,
//         user_token_account,
//     } = SetUpTest::new();
//
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     // // Initialize mint account
//     // let mint_init_ix = spl_token::instruction::initialize_mint(
//     //     &spl_token::ID,
//     //     &mint.pubkey(),
//     //     &admin.pubkey(),
//     //     None,
//     //     9,
//     // )
//     //     .unwrap();
//     // let mint_init_tx = Transaction::new_signed_with_payer(
//     //     &[mint_init_ix],
//     //     Some(&signer.pubkey()),
//     //     &[&signer, &mint],
//     //     recent_blockhash,
//     // );
//     // banks_client.process_transaction(mint_init_tx).await.unwrap();
//
//     // // Initialize user token account
//     // let user_token_init_ix = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &user_token_account.pubkey(),
//     //     &mint.pubkey(),
//     //     &admin.pubkey(),
//     // )
//     //     .unwrap();
//     // let transaction = Transaction::new_signed_with_payer(
//     //     &[user_token_init_ix],
//     //     Some(&payer.pubkey()),
//     //     &[&payer, &user_token_account],
//     //     recent_blockhash,
//     // );
//     // banks_client.process_transaction(transaction).await.unwrap();
//
//
//     // 设置铸币金额
//     let mint_amount: u64 = 1_000_000_000;
//
//     // 调用 faucet 函数
//     let faucet_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::Faucet {
//             mint: mint.pubkey(),
//             user_token_account: user_token_account.pubkey(),
//             admin: admin.pubkey(),
//             token_program: token::ID,
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::Faucet { amount: mint_amount }.data(),
//     };
//
//     let faucet_tx = Transaction::new_signed_with_payer(
//         &[faucet_ix],
//         Some(&admin.pubkey()),
//         &[&admin],
//         recent_blockhash,
//     );
//     banks_client.process_transaction(faucet_tx).await.unwrap();
//
//     // Check the balance of the user token account
//     // let user_token_account_data = banks_client
//     //     .get_account(user_token_account.pubkey())
//     //     .await
//     //     .unwrap()
//     //     .unwrap();
//
//     // let user_token_account_info = TokenAccount::unpack(&user_token_account_data.data).unwrap();
//     // assert_eq!(user_token_account_info.amount, mint_amount);
// }
//
// pub struct SetUpTest {
//     pub program_id: Pubkey,
//     pub pt: ProgramTest,
//     pub signer: Keypair,
//     pub admin: Keypair,
//     pub mint: Keypair,
//     pub user_token_account: Keypair,
// }
//
// impl SetUpTest {
//     pub fn new() -> Self {
//         let program_id = spl_stake::ID;
//         let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
//         pt.set_compute_max_units(1200_000);
//
//         let mut accounts: Vec<Keypair> = Vec::new();
//         let signer = Keypair::new();
//         accounts.push(signer.insecure_clone());
//         let admin = Keypair::new();
//         accounts.push(admin.insecure_clone());
//         // 创建 mint 账户
//         let mint = Keypair::new();
//         accounts.push(mint.insecure_clone());
//         let user_token_account = Keypair::new();
//         accounts.push(user_token_account.insecure_clone());
//
//         for account in accounts {
//             //create a new account and fund with 1 SOL
//             pt.add_account(
//                 account.pubkey(),
//                 solana_sdk::account::Account {
//                     lamports: 1_000_000_000,
//                     ..solana_sdk::account::Account::default()
//                 },
//             );
//         }
//
//         Self {
//             program_id,
//             pt,
//             signer,
//             admin,
//             mint,
//             user_token_account,
//         }
//     }
// }
//
//
// pub async fn load_and_deserialize<T: AccountDeserialize>(
//     mut banks_client: BanksClient,
//     address: Pubkey,
// ) -> T {
//     let account = banks_client
//         .get_account(address)
//         .await
//         .unwrap()
//         .unwrap();
//
//     T::try_deserialize(&mut account.data.as_slice()).unwrap()
// }
//
