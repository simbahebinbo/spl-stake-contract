// // use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
// // use anchor_spl::token;
// // use anchor_spl::token::{Mint, TokenAccount};
// // use solana_program::instruction::Instruction;
// // use solana_program::pubkey::Pubkey;
// // use solana_program::system_program;
// // use solana_program_test::{BanksClient, ProgramTest};
// // use solana_sdk::account::Account;
// // use solana_sdk::signature::Keypair;
// // use solana_sdk::signer::Signer;
// // use solana_sdk::transaction::Transaction;
// //
// // use spl_stake::UserAccount;
// //
// // #[tokio::test]
// // async fn test_deposit() {
// //     let SetUpTest {
// //         program_id,
// //         pt,
// //         admin,
// //         staking_account,
// //         user,
// //         user_account,
// //         staking_token_account,
// //         mint,
// //         user_token_account,
// //     } = SetUpTest::new();
// //
// //     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
// //
// //     // 调用 reset_user_account 指令来初始化 user_account
// //     let reset_user_account_ix = Instruction {
// //         program_id,
// //         accounts: spl_stake::accounts::ResetUserAccount {
// //             user_account: user_account.pubkey(),
// //             user: user.pubkey(),
// //             system_program: system_program::ID,
// //         }
// //             .to_account_metas(None),
// //         data: spl_stake::instruction::ResetUserAccount {}.data(),
// //     };
// //
// //     let reset_user_account_tx = Transaction::new_signed_with_payer(
// //         &[reset_user_account_ix],
// //         Some(&user.pubkey()),
// //         &[&user, &user_account],
// //         recent_blockhash,
// //     );
// //
// //     banks_client.process_transaction(reset_user_account_tx).await;
// //
// //     // 反序列化并检查用户账户
// //     let user_account_data: UserAccount = load_and_deserialize(
// //         banks_client.clone(),
// //         user_account.pubkey(),
// //     ).await;
// //
// //     // 确保用户账户的余额已更新
// //     assert_eq!(user_account_data.balance, 0);
// //
// //     // 创建并发送初始化 mint 的指令
// //     let init_mint_ix = token::spl_token::instruction::initialize_mint(
// //         &token::ID,
// //         &mint,
// //         &admin.pubkey(),
// //         None,
// //         9, // decimals
// //     ).unwrap();
// //
// //     let tx = Transaction::new_signed_with_payer(
// //         &[init_mint_ix],
// //         Some(&admin.pubkey()),
// //         &[&admin],
// //         recent_blockhash,
// //     );
// //
// //     banks_client.process_transaction(tx).await.unwrap();
// //
// //     // 创建并发送初始化 token account 的指令
// //     let init_token_account_ix = token::spl_token::instruction::initialize_account(
// //         &token::ID,
// //         &user_token_account,
// //         &mint,
// //         &admin.pubkey(),
// //     ).unwrap();
// //
// //     let init_token_account_tx = Transaction::new_signed_with_payer(
// //         &[init_token_account_ix],
// //         Some(&admin.pubkey()),
// //         &[&admin],
// //         recent_blockhash,
// //     );
// //
// //     banks_client.process_transaction(init_token_account_tx).await.unwrap();
// //
// //     //
// //     // // 设置铸币金额
// //     // let mint_amount: u64 = 1_000_000_000;
// //     //
// //     // // 调用 faucet 函数
// //     // let faucet_ix = Instruction {
// //     //     program_id: program_id,
// //     //     accounts: spl_stake::accounts::Faucet {
// //     //         mint: mint,
// //     //         user_token_account: user_token_account,
// //     //         admin: admin.pubkey(),
// //     //         token_program: token::ID,
// //     //     }
// //     //         .to_account_metas(None),
// //     //     data: spl_stake::instruction::Faucet { amount: mint_amount }.data(),
// //     // };
// //     //
// //     // let faucet_tx = Transaction::new_signed_with_payer(
// //     //     &[faucet_ix],
// //     //     Some(&admin.pubkey()),
// //     //     &[&admin],
// //     //     recent_blockhash,
// //     // );
// //     // banks_client.process_transaction(faucet_tx).await.unwrap();
// //     //
// //     // // Check the balance of the user token account
// //     // let token_account_data: TokenAccount = load_and_deserialize(
// //     //     banks_client.clone(),
// //     //     user_token_account,
// //     // ).await;
// //     //
// //     // assert_eq!(token_account_data.amount, mint_amount);
// //     //
// //     // // 调用 initialize 指令来初始化 staking_account
// //     // let initialize_ix = Instruction {
// //     //     program_id: program_id,
// //     //     accounts: spl_stake::accounts::Initialize {
// //     //         staking_account: staking_account.pubkey(),
// //     //         admin: admin.pubkey(),
// //     //         system_program: system_program::ID,
// //     //     }
// //     //         .to_account_metas(None),
// //     //     data: spl_stake::instruction::Initialize { admin: admin.pubkey() }.data(),
// //     // };
// //     //
// //     // let initialize_tx = Transaction::new_signed_with_payer(
// //     //     &[initialize_ix],
// //     //     Some(&admin.pubkey()),
// //     //     &[&admin, &staking_account],
// //     //     recent_blockhash,
// //     // );
// //     //
// //     // banks_client.process_transaction(initialize_tx).await.unwrap();
// //     //
// //     // // 设置 supported_token
// //     // let supported_token = Pubkey::new_unique();
// //     //
// //     // let set_supported_token_ix = Instruction {
// //     //     program_id: program_id,
// //     //     accounts: spl_stake::accounts::SetSupportedToken {
// //     //         staking_account: staking_account.pubkey(),
// //     //         admin: admin.pubkey(),
// //     //     }
// //     //         .to_account_metas(None),
// //     //     data: spl_stake::instruction::SetSupportedToken { mint: supported_token }.data(),
// //     // };
// //     //
// //     // let set_supported_token_tx = Transaction::new_signed_with_payer(
// //     //     &[set_supported_token_ix],
// //     //     Some(&admin.pubkey()),
// //     //     &[&admin],
// //     //     recent_blockhash,
// //     // );
// //     //
// //     // banks_client.process_transaction(set_supported_token_tx).await.unwrap();
// //     //
// //     // // 检查 staking_account 是否正确设置了 supported_token
// //     // let staking_account_data: StakingAccount = load_and_deserialize(
// //     //     banks_client.clone(),
// //     //     staking_account.pubkey(),
// //     // ).await;
// //     //
// //     // assert_eq!(staking_account_data.supported_token, supported_token);
// //
// //
// //     // 设置存款金额
// //     let deposit_amount: u64 = 1000;
// //
// //     // 调用存款函数
// //     let deposit_ix = Instruction {
// //         program_id: program_id,
// //         accounts: spl_stake::accounts::Deposit {
// //             user_account: user_account.pubkey(),
// //             user: user.pubkey(),
// //             user_token_account: user_token_account,
// //             staking_token_account: staking_token_account.pubkey(),
// //             token_program: token::ID,
// //         }
// //             .to_account_metas(None),
// //         data: spl_stake::instruction::Deposit { amount: deposit_amount }.data(),
// //     };
// //
// //     let deposit_tx = Transaction::new_signed_with_payer(
// //         &[deposit_ix],
// //         Some(&user.pubkey()),
// //         &[&user],
// //         recent_blockhash,
// //     );
// //
// //     banks_client.process_transaction(deposit_tx).await.unwrap();
// //
// //     // 反序列化并检查用户账户
// //     let user_account_data: UserAccount = load_and_deserialize(
// //         banks_client.clone(),
// //         user_account.pubkey(),
// //     ).await;
// //
// //     // 确保用户账户的余额已更新
// //     assert_eq!(user_account_data.balance, deposit_amount);
// // }
// //
// // pub struct SetUpTest {
// //     pub program_id: Pubkey,
// //     pub pt: ProgramTest,
// //     pub admin: Keypair,
// //     pub staking_account: Keypair,
// //     pub user: Keypair,
// //     pub user_account: Keypair,
// //     pub staking_token_account: Keypair,
// //     pub mint: Pubkey,
// //     pub user_token_account: Pubkey,
// // }
// //
// // impl SetUpTest {
// //     pub fn new() -> Self {
// //         let program_id = spl_stake::ID;
// //         let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
// //         pt.set_compute_max_units(1200_000);
// //
// //         let mut accounts: Vec<Keypair> = Vec::new();
// //         let admin = Keypair::new();
// //         accounts.push(admin.insecure_clone());
// //         let staking_account = Keypair::new();
// //         accounts.push(staking_account.insecure_clone());
// //         // 创建用户和质押账户
// //         let user = Keypair::new();
// //         accounts.push(user.insecure_clone());
// //         let user_account = Keypair::new();
// //         accounts.push(user_account.insecure_clone());
// //         let staking_token_account = Keypair::new();
// //         accounts.push(staking_token_account.insecure_clone());
// //
// //         for account in accounts {
// //             //create a new account and fund with 1 SOL
// //             pt.add_account(
// //                 account.pubkey(),
// //                 Account {
// //                     lamports: 1_000_000_000,
// //                     ..Account::default()
// //                 },
// //             );
// //         }
// //
// //         // 添加mint账户
// //         let mint = Pubkey::new_unique();
// //         pt.add_account(
// //             mint,
// //             Account {
// //                 lamports: 1000000000000,
// //                 data: vec![0; Mint::LEN],
// //                 owner: token::ID,
// //                 executable: false,
// //                 rent_epoch: 0,
// //             },
// //         );
// //
// //         // 添加token账户
// //         let user_token_account = Pubkey::new_unique();
// //         pt.add_account(
// //             user_token_account,
// //             Account {
// //                 lamports: 10000000000000,
// //                 data: vec![0; TokenAccount::LEN],
// //                 owner: token::ID,
// //                 executable: false,
// //                 rent_epoch: 0,
// //             },
// //         );
// //
// //         Self {
// //             program_id,
// //             pt,
// //             admin,
// //             staking_account,
// //             user,
// //             user_account,
// //             staking_token_account,
// //             mint,
// //             user_token_account,
// //         }
// //     }
// // }
// //
// //
// // pub async fn load_and_deserialize<T: AccountDeserialize>(
// //     mut banks_client: BanksClient,
// //     address: Pubkey,
// // ) -> T {
// //     let account = banks_client
// //         .get_account(address)
// //         .await
// //         .unwrap()
// //         .unwrap();
// //
// //     T::try_deserialize(&mut account.data.as_slice()).unwrap()
// // }
// //
// //
//
// use std::collections::HashMap;
//
// use anchor_lang::InstructionData;
// use anchor_lang::prelude::*;
// use solana_program::instruction::Instruction;
// use solana_program::system_program;
// use solana_program_test::*;
// use solana_sdk::signature::Keypair;
// use solana_sdk::signer::Signer;
// use solana_sdk::transaction::Transaction;
//
// #[tokio::test]
// async fn test_deposit() {
//     let SetUpTest {
//         program_id,
//         pt,
//         admin,
//         staking_account,
//         user,
//         user_token_account,
//         staking_token_account,
//     } = SetUpTest::new();
//
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//
//     // 初始化 StakingAccount
//     let initialize_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::Initialize {
//             staking_account: staking_account.pubkey(),
//             admin: admin.pubkey(),
//             system_program: system_program::ID,
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::Initialize { admin: admin.pubkey() }.data(),
//     };
//
//     let initialize_tx = Transaction::new_signed_with_payer(
//         &[initialize_ix],
//         Some(&admin.pubkey()),
//         &[&admin, &staking_account],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(initialize_tx).await.unwrap();
//
//
//     // 设置 supported_token
//     let supported_token = Pubkey::new_unique();
//
//     let set_supported_token_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::SetSupportedToken {
//             staking_account: staking_account.pubkey(),
//             admin: admin.pubkey(),
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::SetSupportedToken { mint: supported_token }.data(),
//     };
//
//     let set_supported_token_tx = Transaction::new_signed_with_payer(
//         &[set_supported_token_ix],
//         Some(&admin.pubkey()),
//         &[&admin],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(set_supported_token_tx).await.unwrap();
//
//
//     // 创建并初始化用户代币账户
//     let mut fake_token_program = FakeTokenProgram::default();
//     fake_token_program.create_account(&user_token_account.pubkey(), 10000);
//     fake_token_program.create_account(&staking_token_account.pubkey(), 0);
//
//     // 模拟存款
//     // let deposit_amount = 1000;
//     // fake_token_program.transfer(&user_token_account.pubkey(), &staking_token_account.pubkey(), deposit_amount);
//
//     // 设置测试环境中相关的账户数据
//     let deposit_accounts = spl_stake::accounts::Deposit {
//         // user_account: fake_token_program.accounts.get_mut(&user_token_account.pubkey()).unwrap(),
//         user: user.pubkey(),
//         user_token_account: user_token_account.pubkey(),
//         staking_token_account: staking_token_account.pubkey(),
//         token_program: Pubkey::default(), // Mock or fake the token program ID
//     };
//
//     // 模拟存款
//     let deposit_amount = 1000;
//     let deposit_ix = spl_stake::instruction::deposit(
//         &program_id,
//         // &deposit_accounts.user_account.pubkey(),
//         &deposit_accounts.user_token_account.pubkey(),
//         &deposit_accounts.staking_token_account.pubkey(),
//         deposit_amount,
//     );
//
//     let mut transaction = Transaction::new_with_payer(&[deposit_ix], Some(&payer.pubkey()));
//     transaction.sign(&[&payer, &user], recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//
//     // let mut transaction = Transaction::new_with_payer(&[deposit_ix], Some(&payer.pubkey()));
//     // transaction.sign(&[&payer, &user], recent_blockhash);
//     // banks_client.process_transaction(transaction).await?;
//     //
//     //
//     // 模拟存款指令
//     // let deposit_ix = spl_stake::instruction::deposit(
//     //     &program_id,
//     //     &user.pubkey(),
//     //     &user_token_account.pubkey(),
//     //     &staking_token_account.pubkey(),
//     //     deposit_amount,
//     // );
//     // let mut transaction = Transaction::new_with_payer(&[deposit_ix], Some(&payer.pubkey()));
//     // transaction.sign(&[&payer, &user], recent_blockhash);
//     // banks_client.process_transaction(transaction).await;
//
//
//     // 调用存款函数
//     // let deposit_ix = Instruction {
//     //     program_id: program_id,
//     //     accounts: spl_stake::accounts::Deposit {
//     //         // user_account: user_token_account.pubkey(),
//     //         user: user.pubkey(),
//     //         user_token_account: user_token_account.pubkey(),
//     //         staking_token_account: staking_token_account.pubkey(),
//     //         token_program: Pubkey::default(),
//     //     }
//     //         .to_account_metas(None),
//     //     data: spl_stake::instruction::Deposit { amount: deposit_amount }.data(),
//     // };
//     //
//     // let deposit_tx = Transaction::new_signed_with_payer(
//     //     &[deposit_ix],
//     //     Some(&user.pubkey()),
//     //     &[&user],
//     //     recent_blockhash,
//     // );
//     //
//     // banks_client.process_transaction(deposit_tx).await.unwrap();
//     //
//     //
//     // // 检查账户余额
//     // let user_token_balance = fake_token_program.get_balance(&user_token_account.pubkey());
//     // let staking_token_balance = fake_token_program.get_balance(&staking_token_account.pubkey());
//     //
//     // assert_eq!(user_token_balance, 9000); // 存款后用户账户余额应为 9000
//     // assert_eq!(staking_token_balance, 1000); // 存款后质押账户余额应为 1000
// }
//
// pub struct SetUpTest {
//     pub program_id: Pubkey,
//     pub pt: ProgramTest,
//     pub admin: Keypair,
//     pub staking_account: Keypair,
//     pub user: Keypair,
//     pub user_token_account: Keypair,
//     pub staking_token_account: Keypair,
// }
//
// impl SetUpTest {
//     pub fn new() -> Self {
//         let program_id = spl_stake::ID;
//         let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
//         pt.set_compute_max_units(1200_000);
//
//         let mut accounts: Vec<Keypair> = Vec::new();
//         let admin = Keypair::new();
//         accounts.push(admin.insecure_clone());
//         let staking_account = Keypair::new();
//         accounts.push(staking_account.insecure_clone());
//         let user = Keypair::new();
//         accounts.push(user.insecure_clone());
//         let user_token_account = Keypair::new();
//         accounts.push(user_token_account.insecure_clone());
//         let staking_token_account = Keypair::new();
//         accounts.push(staking_token_account.insecure_clone());
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
//             admin,
//             staking_account,
//             user,
//             user_token_account,
//             staking_token_account,
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
// // 模拟代币账户的余额
// #[derive(Debug, Default, Clone)]
// struct FakeTokenAccount {
//     pub balance: u64,
// }
//
// // 模拟代币账户的处理
// #[derive(Debug, Default)]
// struct FakeTokenProgram {
//     accounts: HashMap<Pubkey, FakeTokenAccount>,
// }
//
// impl FakeTokenProgram {
//     pub fn transfer(&mut self, from: &Pubkey, to: &Pubkey, amount: u64) {
//         let mut from_account = self.accounts.get_mut(from).unwrap().clone();
//         let mut to_account = self.accounts.get_mut(to).unwrap().clone();
//
//         from_account.balance -= amount;
//         to_account.balance += amount;
//     }
//
//     pub fn create_account(&mut self, pubkey: &Pubkey, balance: u64) {
//         self.accounts.insert(pubkey.clone(), FakeTokenAccount { balance });
//     }
//
//     pub fn get_balance(&self, pubkey: &Pubkey) -> u64 {
//         self.accounts.get(pubkey).map_or(0, |acc| acc.balance)
//     }
// }
