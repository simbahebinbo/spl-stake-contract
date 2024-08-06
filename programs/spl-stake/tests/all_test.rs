use anchor_lang::InstructionData;
use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;
use solana_program::system_program;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[tokio::test]
async fn test_initialize() {
    let SetUpTest {
        program_id,
        pt,
        signer,
        admin,
        staking_account,
        mint,
        user,
        user_token_account,
    } = SetUpTest::new();


    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    let initialize_ix = Instruction {
        program_id: program_id,
        accounts: spl_stake::accounts::Initialize {
            staking_account: staking_account.pubkey(),
            admin: admin.pubkey(),
            system_program: system_program::ID,
        }
            .to_account_metas(None),
        data: spl_stake::instruction::Initialize { admin: admin.pubkey() }.data(),
    };

    let signed_txs = Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&admin.pubkey()),
        &[&admin, &staking_account],
        recent_blockhash,
    );

    banks_client.process_transaction(signed_txs).await.unwrap();
}


// #[tokio::test]
// async fn test_faucet() {
//     let SetUpTest {
//         program_id,
//         pt,
//         signer,
//         admin,
//         staking_account,
//         mint,
//         user,
//         user_token_account,
//     } = SetUpTest::new();
//
//     let (mut banks_client, payer, recent_blockhash) = pt.start().await;
//
//     let mint = Keypair::new();
//     let admin = Keypair::new();
//     let user = Keypair::new();
//     let user_token_account = Keypair::new();
//
//     // Create mint account
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &mint.pubkey(),
//                 Rent::default().minimum_balance(Mint::LEN),
//                 Mint::LEN as u64,
//                 &token::ID,
//             ),
//             token::instruction::initialize_mint(
//                 &token::ID,
//                 &mint.pubkey(),
//                 &admin.pubkey(),
//                 None,
//                 9,
//             )
//                 .unwrap(),
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &user_token_account.pubkey(),
//                 Rent::default().minimum_balance(TokenAccount::LEN),
//                 TokenAccount::LEN as u64,
//                 &token::ID,
//             ),
//             token::instruction::initialize_account(
//                 &token::ID,
//                 &user_token_account.pubkey(),
//                 &mint.pubkey(),
//                 &user.pubkey(),
//             )
//                 .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//
//     transaction.sign(
//         &[&payer, &mint, &user_token_account],
//         recent_blockhash,
//     );
//     banks_client.process_transaction(transaction).await?;
//
//     // Call faucet function
//     let mut transaction = Transaction::new_with_payer(
//         &[staking_instruction::faucet(
//             staking_accounts::Faucet {
//                 mint: mint.pubkey(),
//                 user_token_account: user_token_account.pubkey(),
//                 admin: admin.pubkey(),
//                 token_program: token::ID,
//             },
//             1000000000, // 1 token with 9 decimal places
//         )],
//         Some(&payer.pubkey()),
//     );
//
//     transaction.sign(&[&payer, &admin], recent_blockhash);
//     banks_client.process_transaction(transaction).await?;
//
//     Ok(())
// }

pub struct SetUpTest {
    pub program_id: Pubkey,
    pub pt: ProgramTest,
    pub signer: Keypair,
    pub admin: Keypair,
    pub staking_account: Keypair,
    pub mint: Keypair,
    pub user: Keypair,
    pub user_token_account: Keypair,
}

impl SetUpTest {
    pub fn new() -> Self {
        let program_id = spl_stake::ID;
        let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
        pt.set_compute_max_units(1200_000);

        let mut accounts: Vec<Keypair> = Vec::new();
        let signer = Keypair::new();
        accounts.push(signer.insecure_clone());
        let admin = Keypair::new();
        accounts.push(admin.insecure_clone());
        let staking_account = Keypair::new();
        accounts.push(staking_account.insecure_clone());
        let mint = Keypair::new();
        accounts.push(mint.insecure_clone());
        let user = Keypair::new();
        accounts.push(user.insecure_clone());
        let user_token_account = Keypair::new();
        accounts.push(user_token_account.insecure_clone());

        for account in accounts {
            //create a new account and fund with 1 SOL
            pt.add_account(
                account.pubkey(),
                solana_sdk::account::Account {
                    lamports: 1_000_000_000,
                    ..solana_sdk::account::Account::default()
                },
            );
        }

        Self {
            program_id,
            pt,
            signer,
            admin,
            staking_account,
            mint,
            user,
            user_token_account,
        }
    }
}


pub async fn load_and_deserialize<T: AccountDeserialize>(
    mut banks_client: BanksClient,
    address: Pubkey,
) -> T {
    let account = banks_client
        .get_account(address)
        .await
        .unwrap()
        .unwrap();

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}
