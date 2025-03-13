#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("Df7xQK8FXzyRinP8XXZwQym7CG998FrUkogtDHCyjN43");

#[program]
pub mod call_it_a_comeback {
    use super::*;

    pub fn initialize_cic_token(ctx: Context<InitializeCicToken>) -> Result<()> {
        msg!("Program invoked. Initializing CIC token...");
        msg!(
            "  New token mint public key: {}",
            &ctx.accounts.mint.key().to_string()
        );

        // Initialize the mint with 9 decimals
        let cpi_accounts = token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 9, &ctx.accounts.payer.key(), None)?; // Payer as mint authority

        // Mint 10B CIC to the payer's token account
        msg!(
            "Minting 10 billion CIC to: {}",
            &ctx.accounts.payer_token_account.key().to_string()
        );
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.payer_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 10_000_000_000_000_000_000)?; // 100B CIC with 9 decimals

        msg!("CIC token created and minted successfully.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCicToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,                    // The account paying for and owning the token
    #[account(mut)]
    pub mint: Account<'info, Mint>,              // The new token mint
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>, // Payer's ATA to receive CIC
    pub token_program: Program<'info, Token>,    // SPL Token program
    pub rent: Sysvar<'info, Rent>,               // Rent sysvar
    pub system_program: Program<'info, System>,  // System program
}
