use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("29iiLtNregFkwH4n4K95GrKYcGUGC3F6D5thPE2jWQQs");

#[program]
pub mod spl_token {
    use super::*;

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.payer.key();
        vault.spl_token_mint_bump = *ctx.bumps.get("spl_token_mint").unwrap();
        vault.bump = *ctx.bumps.get("vault").unwrap();
        vault.spl_token_mint = ctx.accounts.spl_token_mint.key();
  
        Ok(())
   }
}

#[derive(Accounts)]
pub struct Initialize {}



#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump,
        payer = payer,
        mint::authority = payer,
        mint::decimals = 0,
        mint::freeze_authority = payer
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 2

    pub system_program: Program<'info, System>, // ---> 3
    pub token_program: Program<'info, Token>,   // ---> 4
    // this is required for spl token mint
    pub rent: Sysvar<'info, Rent>, // ---> 5

    #[account(
        init, 
        space = 8 + Vault::LEN,
        seeds = [
            b"vault"
        ],
        bump,
        payer = payer 
    )]
    pub vault : Account<'info, Vault>, // ---> 6
}


// Store the state 
#[account]
pub struct Vault {
    bump : u8, //1
    spl_token_mint_bump:u8, // 1
    authority : Pubkey, //32
    spl_token_mint : Pubkey //32
}

impl Vault {
    pub const LEN: usize =1 + 1 + 32 + 32;
}