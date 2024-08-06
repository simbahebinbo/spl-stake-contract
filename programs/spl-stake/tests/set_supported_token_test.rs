use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_program_test::{BanksClient, ProgramTest};
use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

use spl_stake::StakingAccount;

#[tokio::test]
async fn test_set_supported_token() {
    let SetUpTest {
        program_id,
        pt,
        signer,
        staking_account,
    } = SetUpTest::new();

    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    // 调用 initialize 指令来初始化 staking_account
    let initialize_ix = Instruction {
        program_id: program_id,
        accounts: spl_stake::accounts::Initialize {
            staking_account: staking_account.pubkey(),
            admin: signer.pubkey(),
            system_program: system_program::ID,
        }
            .to_account_metas(None),
        data: spl_stake::instruction::Initialize { admin: signer.pubkey() }.data(),
    };

    let initialize_tx = Transaction::new_signed_with_payer(
        &[initialize_ix],
        Some(&signer.pubkey()),
        &[&signer, &staking_account],
        recent_blockhash,
    );

    banks_client.process_transaction(initialize_tx).await.unwrap();

    // 设置 supported_token
    let supported_token = Pubkey::new_unique();

    let set_supported_token_ix = Instruction {
        program_id: program_id,
        accounts: spl_stake::accounts::SetSupportedToken {
            staking_account: staking_account.pubkey(),
            admin: signer.pubkey(),
        }
            .to_account_metas(None),
        data: spl_stake::instruction::SetSupportedToken { mint: supported_token }.data(),
    };

    let set_supported_token_tx = Transaction::new_signed_with_payer(
        &[set_supported_token_ix],
        Some(&signer.pubkey()),
        &[&signer],
        recent_blockhash,
    );

    banks_client.process_transaction(set_supported_token_tx).await.unwrap();

    // 检查 staking_account 是否正确设置了 supported_token
    let staking_account_data: StakingAccount = load_and_deserialize(
        banks_client.clone(),
        staking_account.pubkey(),
    ).await;

    assert_eq!(staking_account_data.supported_token, supported_token);
}

pub struct SetUpTest {
    pub program_id: Pubkey,
    pub pt: ProgramTest,
    pub signer: Keypair,
    pub staking_account: Keypair,
}

impl SetUpTest {
    pub fn new() -> Self {
        let program_id = spl_stake::ID;
        let mut pt: ProgramTest = ProgramTest::new("spl_stake", program_id, None);
        pt.set_compute_max_units(1200_000);

        let mut accounts: Vec<Keypair> = Vec::new();
        let signer = Keypair::new();
        accounts.push(signer.insecure_clone());
        let staking_account = Keypair::new();
        accounts.push(staking_account.insecure_clone());


        for account in accounts {
            //create a new account and fund with 1 SOL
            pt.add_account(
                account.pubkey(),
                Account {
                    lamports: 1_000_000_000,
                    ..Account::default()
                },
            );
        }

        Self {
            program_id,
            pt,
            signer,
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

