// use anchor_lang::InstructionData;
// use anchor_lang::prelude::*;
// use solana_program::instruction::Instruction;
// use solana_program::system_program;
// use solana_program_test::*;
// use solana_sdk::{
//     signature::Signer,
//     transaction::Transaction,
// };
// use solana_sdk::signature::Keypair;
//
// use spl_stake::{self, UserAccount};
//
// #[tokio::test]
// async fn test_initialize_user_account() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         user,
//         user_account,
//     } = SetUpTest::new();
//
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     let initialize_user_account_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::InitializeUserAccount {
//             user_account: user_account.pubkey(),
//             user: user.pubkey(),
//             system_program: system_program::ID,
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::InitializeUserAccount {}.data(),
//     };
//
//     let initialize_user_account_tx = Transaction::new_signed_with_payer(
//         &[initialize_user_account_ix],
//         Some(&user.pubkey()),
//         &[&user, &user_account],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(initialize_user_account_tx).await.unwrap();
//
//     // 反序列化并检查用户账户
//     let user_account_data: UserAccount = load_and_deserialize(
//         banks_client.clone(),
//         user_account.pubkey(),
//     ).await;
//
//     // 确保用户账户的余额已更新
//     assert_eq!(user_account_data.balance, 0);
// }
//
//
// pub struct SetUpTest {
//     pub program_id: Pubkey,
//     pub pt: ProgramTest,
//     pub signer: Keypair,
//     pub user: Keypair,
//     pub user_account: Keypair,
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
//         let user = Keypair::new();
//         accounts.push(user.insecure_clone());
//         let user_account = Keypair::new();
//         accounts.push(user_account.insecure_clone());
//
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
//             user,
//             user_account,
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
//
//
