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
    } = SetUpTest::new();


    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    let initialize_ix = Instruction {
        program_id: program_id,
        accounts: spl_stake::accounts::Initialize {
            staking_account: staking_account.pubkey(),
            signer: signer.pubkey(),
            system_program: system_program::ID,
        }
            .to_account_metas(None),
        data: spl_stake::instruction::Initialize { admin: admin.pubkey() }.data(),
    };

    let signed_txs = Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&signer.pubkey()),
        &[&signer, &staking_account],
        recent_blockhash,
    );
    
    banks_client.process_transaction(signed_txs).await.unwrap();
}


pub struct SetUpTest {
    pub program_id: Pubkey,
    pub pt: ProgramTest,
    pub signer: Keypair,
    pub admin: Keypair,
    pub staking_account: Keypair,
}

impl SetUpTest {
    pub fn new() -> Self {
        let program_id = spl_stake::ID;
        let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
        pt.set_compute_max_units(1200_000);

        //create a new signer and fund with 1 SOL
        let signer = Keypair::new();
        pt.add_account(
            signer.pubkey(),
            solana_sdk::account::Account {
                lamports: 1_000_000_000,
                ..solana_sdk::account::Account::default()
            },
        );

        let staking_account = Keypair::new();
        pt.add_account(
            staking_account.pubkey(),
            solana_sdk::account::Account {
                lamports: 1_000_000_000,
                ..solana_sdk::account::Account::default()
            },
        );

        let admin = Keypair::new();
        pt.add_account(
            admin.pubkey(),
            solana_sdk::account::Account {
                lamports: 1_000_000_000,
                ..solana_sdk::account::Account::default()
            },
        );

        Self {
            program_id,
            pt,
            signer,
            admin,
            staking_account,
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
