use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("4gzQgfWr98rvu5WqJZophHUS42rMWAz6yntxyEdqcQNY");

// const PDA_SEED: &[u8] = b"hello";

// #[derive(AnchorSerialize, AnchorDeserialize)]
// pub struct UserInfoParams {
//     pub name: String,
//     pub age: u8,
// }

#[program]
pub mod spl_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.admin = admin;
        Ok(())
    }


    // pub fn set_supported_token(ctx: Context<SetSupportedToken>, mint: Pubkey) -> Result<()> {
    //     let staking_account = &mut ctx.accounts.staking_account;
    //     require!(staking_account.admin == *ctx.accounts.admin.key, CustomError::Unauthorized);
    //     staking_account.supported_token = mint;
    //     Ok(())
    // }
    //
    // pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    //     let user_account = &mut ctx.accounts.user_account;
    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.user_token_account.to_account_info(),
    //         to: ctx.accounts.staking_token_account.to_account_info(),
    //         authority: ctx.accounts.user.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     token::transfer(cpi_ctx, amount)?;
    //     user_account.balance += amount;
    //     Ok(())
    // }
    //
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    //     let user_account = &mut ctx.accounts.user_account;
    //     require!(user_account.balance >= amount, CustomError::InsufficientFunds);
    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.staking_token_account.to_account_info(),
    //         to: ctx.accounts.user_token_account.to_account_info(),
    //         authority: ctx.accounts.admin.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     token::transfer(cpi_ctx, amount)?;
    //     user_account.balance -= amount;
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 32)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetSupportedToken<'info> {
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingAccount {
    pub admin: Pubkey,
    pub supported_token: Pubkey,
}

#[account]
pub struct UserAccount {
    pub balance: u64,
}

// #[error]
// pub enum CustomError {
//     #[msg("Unauthorized")]
//     Unauthorized,
//     #[msg("Insufficient Funds")]
//     InsufficientFunds,
// }


// #[program]
// pub mod spl_stake {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>, params: UserInfoParams) -> Result<()> {
//         let user_data = &mut ctx.accounts.data;
//         user_data.name = params.name;
//         user_data.age = params.age;
//         Ok(())
//     }
// }

// #[account]
// pub struct UserInfo {
//     pub name: String,
//     pub age: u8,
// }

// #[derive(Accounts)]
// #[instruction(instruction_data: UserInfoParams)]
// pub struct Initialize<'info> {
//     #[account(
//         init,
//         seeds = [PDA_SEED, authority.key().as_ref()],
//         bump,
//         payer = authority,
//         space = 8 + 4 + instruction_data.name.len() * 2 + 1,
//     )]
//     pub data: Account<'info, UserInfo>,
//     #[account(mut)]
//     pub authority: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
