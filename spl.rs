use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_program, program};

#[program]
mod simple_smart_contract {
    use super::*;

    // Define the struct for the token account.
    pub struct MyTokenAccount {
        pub authority: Pubkey,
        pub amount: u64,
        pub mint: Pubkey,
    }

    // Define the minting function.
    pub fn mint_token(ctx: Context<Auth>, amount: u64) -> ProgramResult {
        // Access the accounts from the context.
        let authority = &ctx.accounts.authority;
        let token_account = &mut ctx.accounts.token_account;

        // Ensure the caller is the authority.
        if authority.key != &token_account.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Mint new tokens to the token account.
        token_account.amount += amount;

        Ok(())
    }

    // Define the transferring function.
    pub fn transfer_token(ctx: Context<Transfer>, amount: u64) -> ProgramResult {
        // Access the accounts from the context.
        let authority = &ctx.accounts.authority;
        let from_account = &mut ctx.accounts.from_account;
        let to_account = &mut ctx.accounts.to_account;

        // Ensure the caller is the authority.
        if authority.key != &from_account.authority {
            return Err(ErrorCode::Unauthorized.into());
        }

        // Ensure the from_account has sufficient balance.
        if from_account.amount < amount {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        // Transfer tokens from the sender to the receiver.
        from_account.amount -= amount;
        to_account.amount += amount;

        Ok(())
    }
}

// Define error codes.
#[error]
pub enum ErrorCode {
    #[msg("Unauthorized: Caller is not authorized.")]
    Unauthorized,
    #[msg("Insufficient balance: Account does not have enough tokens.")]
    InsufficientBalance,
}

// Define the accounts and context for minting.
#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: Account<'info, MyTokenAccount>,
    #[account(mut, payer)]
    pub system_program: AccountInfo<'info>,
}

// Define the accounts and context for transferring.
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub from_account: Account<'info, MyTokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, MyTokenAccount>,
}

