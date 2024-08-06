// use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
// use solana_program::instruction::Instruction;
// use solana_program::program_pack::Pack;
// use solana_program::pubkey::Pubkey;
// use solana_program::system_program;
// use solana_program_test::{BanksClient, ProgramTest};
// use solana_sdk::signature::Keypair;
// use solana_sdk::signer::Signer;
// use solana_sdk::transaction::Transaction;
//
// use spl_stake::{StakingAccount, UserAccount};
//
// #[tokio::test]
// async fn test_deposit() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         admin,
//         staking_account,
//         user,
//         user_account,
//         user_token_account,
//         staking_token_account,
//         user_token_mint,
//     } = SetUpTest::new();
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     // 创建 SPL Token Mint 账户
//     // let user_token_mint = Keypair::new();
//     // let initialize_mint_ix = spl_token::instruction::initialize_mint(
//     //     &spl_token::ID,
//     //     &user_token_mint.pubkey(),
//     //     &admin.pubkey(),
//     //     None,
//     //     9,
//     // ).unwrap();
//     //
//     // let initialize_mint_tx = Transaction::new_signed_with_payer(
//     //     &[initialize_mint_ix],
//     //     Some(&signer.pubkey()),
//     //     &[&signer, &user_token_mint],
//     //     recent_blockhash,
//     // );
//     //
//     // banks_client.process_transaction(initialize_mint_tx).await.unwrap();
//
//     // // 创建和初始化 SPL Token 账户
//     // let initialize_user_token_account_ix = initialize_account(
//     //     &spl_token::ID,
//     //     &user_token_account.pubkey(),
//     //     &user_token_mint.pubkey(),
//     //     &user.pubkey(),
//     // ).unwrap();
//     //
//     // let initialize_user_token_account_tx = Transaction::new_signed_with_payer(
//     //     &[initialize_user_token_account_ix],
//     //     Some(&signer.pubkey()),
//     //     &[&signer, &user_token_account],
//     //     recent_blockhash,
//     // );
//     //
//     // banks_client.process_transaction(initialize_user_token_account_tx).await.unwrap();
//
//
//     // 调用 initialize 指令来初始化 staking_account
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
//     // 检查 staking_account 是否正确设置了 supported_token
//     let staking_account_data: StakingAccount = load_and_deserialize(
//         banks_client.clone(),
//         staking_account.pubkey(),
//     ).await;
//
//     assert_eq!(staking_account_data.supported_token, supported_token);
//
//     // // 初始化 user_account
//     // let initialize_user_account_ix = Instruction {
//     //     program_id: program_id,
//     //     accounts: spl_stake::accounts::InitializeUserAccount {
//     //         user_account: user_account.pubkey(),
//     //         user: user.pubkey(),
//     //         system_program: system_program::ID,
//     //     }
//     //         .to_account_metas(None),
//     //     data: spl_stake::instruction::InitializeUserAccount {}.data(),
//     // };
//     //
//     // let initialize_user_account_tx = Transaction::new_signed_with_payer(
//     //     &[initialize_user_account_ix],
//     //     Some(&user.pubkey()),
//     //     &[&user, &user_account],
//     //     recent_blockhash,
//     // );
//     //
//     // banks_client.process_transaction(initialize_user_account_tx).await.unwrap();
//     //
//
//     // 设置存款金额
//     let deposit_amount: u64 = 1000;
//
//     // 调用存款函数
//     let deposit_ix = Instruction {
//         program_id: program_id,
//         accounts: spl_stake::accounts::Deposit {
//             user_account: user.pubkey(),
//             user: user.pubkey(),
//             user_token_account: user_token_account.pubkey(),
//             staking_token_account: staking_token_account.pubkey(),
//             token_program: spl_token::ID.into(),
//         }
//             .to_account_metas(None),
//         data: spl_stake::instruction::Deposit { amount: deposit_amount }.data(),
//     };
//
//     let deposit_tx = Transaction::new_signed_with_payer(
//         &[deposit_ix],
//         Some(&user.pubkey()),
//         &[&user],
//         recent_blockhash,
//     );
//
//     banks_client.process_transaction(deposit_tx).await.unwrap();
//
//     // 反序列化并检查用户账户
//     let user_account_data: UserAccount = load_and_deserialize(
//         banks_client.clone(),
//         user_account.pubkey(),
//     ).await;
//
//     // 确保用户账户的余额已更新
//     assert_eq!(user_account_data.balance, deposit_amount);
// }
//
// pub struct SetUpTest {
//     pub program_id: Pubkey,
//     pub pt: ProgramTest,
//     pub signer: Keypair,
//     pub admin: Keypair,
//     pub staking_account: Keypair,
//     pub user: Keypair,
//     pub user_account: Keypair,
//     pub user_token_account: Keypair,
//     pub staking_token_account: Keypair,
//     pub user_token_mint: Keypair,
// }
//
// impl SetUpTest {
//     pub fn new() -> Self {
//         let program_id = spl_stake::ID;
//         let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
//         pt.set_compute_max_units(1200_000);
//
//         let mut accounts: Vec<Keypair> = Vec::new();
//
//         let signer = Keypair::new();
//         accounts.push(signer.insecure_clone());
//         let admin = Keypair::new();
//         accounts.push(admin.insecure_clone());
//         let staking_account = Keypair::new();
//         accounts.push(staking_account.insecure_clone());
//         // 创建用户和质押账户
//         let user = Keypair::new();
//         accounts.push(user.insecure_clone());
//         let user_account = Keypair::new();
//         accounts.push(user_account.insecure_clone());
//         let user_token_account = Keypair::new();
//         accounts.push(user_token_account.insecure_clone());
//         let staking_token_account = Keypair::new();
//         accounts.push(staking_token_account.insecure_clone());
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
//         let user_token_mint = Keypair::new();
//         pt.add_account(
//             user_token_mint.pubkey(),
//             solana_sdk::account::Account {
//                 lamports: 1_000_000_000,
//                 data: vec![0; spl_token::state::Mint::LEN],
//                 owner: spl_token::ID,
//                 ..solana_sdk::account::Account::default()
//             },
//         );
//
//         Self {
//             program_id,
//             pt,
//             signer,
//             admin,
//             staking_account,
//             user,
//             user_account,
//             user_token_account,
//             staking_token_account,
//             user_token_mint,
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
