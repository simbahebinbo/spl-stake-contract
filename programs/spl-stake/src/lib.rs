use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("4gzQgfWr98rvu5WqJZophHUS42rMWAz6yntxyEdqcQNY");

#[program]
pub mod spl_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.admin = admin;
        Ok(())
    }

    pub fn set_supported_token(ctx: Context<SetSupportedToken>, mint: Pubkey) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        require!(staking_account.admin == *ctx.accounts.admin.key, StakingError::Unauthorized);
        staking_account.supported_token = mint;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        require!(user_account.amount >= 0, StakingError::InsufficientFunds);
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.staking_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        user_account.amount += amount;

        // Emit the deposit event
        emit!(DepositEvent {
            user: ctx.accounts.user_account.key(),
            amount,
        });

        Ok(())
    }


    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        require!(user_account.amount >= amount, StakingError::InsufficientFunds);
        let cpi_accounts = Transfer {
            from: ctx.accounts.staking_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        user_account.amount -= amount;

        // Emit the withdraw event
        emit!(WithdrawEvent {
            user: ctx.accounts.user_account.key(),
            amount,
        });

        Ok(())
    }

    pub fn faucet(ctx: Context<Faucet>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn reset_user_account(ctx: Context<ResetUserAccount>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.amount = 0;
        Ok(())
    }

    pub fn simulate(ctx: Context<Simulate>) -> Result<()> {
        Ok(())
    }
}


// Define the deposit event
#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
}

// Define the withdraw event
#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = deployer, space = 8 + 32 + 32)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(mut)]
    pub deployer: Signer<'info>,
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

#[derive(Accounts)]
pub struct Faucet<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakingAccount {
    pub admin: Pubkey,
    pub supported_token: Pubkey,
}

#[derive(Accounts)]
pub struct ResetUserAccount<'info> {
    #[account(init_if_needed, payer = admin, space = 8 + UserAccount::LEN)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Simulate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
}

#[account]
pub struct UserAccount {
    pub amount: u64,
}

impl UserAccount {
    pub const LEN: usize = 8 + 8; // 8字节的鉴别符 + 8字节的余额

    pub fn discriminator() -> [u8; 8] {
        *b"usraccnt" // 8字节的鉴别符
    }
}

#[error_code]
pub enum StakingError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
}

