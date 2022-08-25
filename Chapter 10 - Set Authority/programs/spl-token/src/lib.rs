use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
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

    pub fn transfer_mint(ctx: Context<TransferMint>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.spl_token_mint.to_account_info(),
                to: ctx.accounts.payer_mint_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );
        token::mint_to(cpi_context, 10)?; // we are minting 10 tokens
        Ok(())
    }

    pub fn set_mint_authority(ctx: Context<SetMintTokenAuthority>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: ctx.accounts.payer.to_account_info(),
                account_or_mint: ctx.accounts.spl_token_mint.to_account_info(),
            },
        );
        token::set_authority(
            cpi_context,
            AuthorityType::MintTokens,
            Some(ctx.accounts.another_authority.key()),
        )?;
        Ok(())
    }

    pub fn set_freeze_account_authority(ctx: Context<SetFreezeAccountAuthority>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: ctx.accounts.payer.to_account_info(),
                account_or_mint: ctx.accounts.spl_token_mint.to_account_info(),
            },
        );
        token::set_authority(
            cpi_context,
            AuthorityType::FreezeAccount,
            Some(ctx.accounts.another_authority.key()),
        )?;
        Ok(())
    }

    pub fn set_account_owner_authority(ctx: Context<SetAccountOwnerAuthority>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: ctx.accounts.payer.to_account_info(),
                account_or_mint: ctx.accounts.payer_mint_ata.to_account_info(),
            },
        );
        token::set_authority(
            cpi_context,
            AuthorityType::AccountOwner,
            Some(ctx.accounts.another_authority.key()),
        )?;
        Ok(())
    }

    pub fn set_close_account_authority(ctx: Context<SetCloseAccountAuthority>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: ctx.accounts.another_authority.to_account_info(),
                account_or_mint: ctx.accounts.another_mint_ata.to_account_info(),
            },
        );
        token::set_authority(
            cpi_context,
            AuthorityType::CloseAccount,
            Some(ctx.accounts.payer.key()),
        )?;
        Ok(())
    }
}

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
    pub vault: Account<'info, Vault>, // ---> 6
}

// Store the state
#[account]
pub struct Vault {
    bump: u8,                //1
    spl_token_mint_bump: u8, // 1
    authority: Pubkey,       //32
    spl_token_mint: Pubkey,  //32
}

impl Vault {
    pub const LEN: usize = 1 + 1 + 32 + 32;
}

// Transfer mint context
#[derive(Accounts)]
pub struct TransferMint<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = spl_token_mint,
        associated_token::authority = payer
    )]
    pub payer_mint_ata: Box<Account<'info, TokenAccount>>, // --> 3

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 4

    pub system_program: Program<'info, System>, // ---> 5
    pub token_program: Program<'info, Token>,   // ---> 6

    pub rent: Sysvar<'info, Rent>, // ---> 7

    pub associated_token_program: Program<'info, AssociatedToken>, // ---> 8
}

// Set Mint Token Authority context
#[derive(Accounts)]
pub struct SetMintTokenAuthority<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 3

    pub another_authority: Signer<'info>, // ---> 4

    pub system_program: Program<'info, System>, // ---> 5
    pub token_program: Program<'info, Token>,   // ---> 6
}

// Set Freeze Account Authority context
#[derive(Accounts)]
pub struct SetFreezeAccountAuthority<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 3

    pub another_authority: Signer<'info>, // ---> 4

    pub system_program: Program<'info, System>, // ---> 5
    pub token_program: Program<'info, Token>,   // ---> 6
}

// Set Account Owner Authority context
#[derive(Accounts)]
pub struct SetAccountOwnerAuthority<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 3

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = payer
    )]
    pub payer_mint_ata: Account<'info, TokenAccount>, // ---> 4

    pub another_authority: Signer<'info>, // ---> 5

    pub system_program: Program<'info, System>, // ---> 6
    pub token_program: Program<'info, Token>,   // ---> 7
}


// Set Close Account Authority context
#[derive(Accounts)]
pub struct SetCloseAccountAuthority<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 3

    #[account(
        init,
        associated_token::mint = spl_token_mint,
        associated_token::authority = another_authority,
        payer = payer
       
    )]
    pub another_mint_ata: Account<'info, TokenAccount>, // ---> 4

    pub another_authority: Signer<'info>, // ---> 5

    pub system_program: Program<'info, System>, // ---> 6
    pub token_program: Program<'info, Token>,   // ---> 7

    pub associated_token_program : Program<'info, AssociatedToken>, // ---> 8,

    pub rent: Sysvar<'info, Rent>, // ---> 9
}
