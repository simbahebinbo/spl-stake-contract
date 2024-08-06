// use anchor_lang::prelude::*;
// use solana_program_test::*;
// use solana_sdk::{
//     signature::Keypair,
//     signature::Signer
//     ,
// };
// use spl_token::{self, ID as SPL_TOKEN_ID};
//
// #[tokio::test]
// async fn test_mint() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         admin,
//         mint,
//     } = SetUpTest::new();
//
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     // // Initialize mint account
//     // let mint_init_ix = spl_token::instruction::initialize_mint(
//     //     &SPL_TOKEN_ID,
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
// }
//
//
// pub struct SetUpTest {
//     pub program_id: Pubkey,
//     pub pt: ProgramTest,
//     pub signer: Keypair,
//     pub admin: Keypair,
//     pub mint: Keypair,
// }
//
// impl SetUpTest {
//     pub fn new() -> Self {
//         let program_id = spl_stake::ID;
//         let mut pt = ProgramTest::new("spl_stake", program_id, None);
//
//         // 显式添加 spl_token 程序路径
//         pt.add_program(
//             "spl_token",
//             SPL_TOKEN_ID,
//             processor!(spl_token::processor::Processor::process),
//         );
//
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
//             admin,
//             mint,
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
