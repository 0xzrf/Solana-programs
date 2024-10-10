use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token},
    metadata::{
        create_metadata_accounts_v3,mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3, Metadata
    }
};

declare_id!("8vaEmpFzZKoh6zeMFSn7jdzBcbsJWqCgy9vkjTajQwvk");

#[program]
pub mod mock {
    use super::*;

    pub fn create_token(ctx: Context<InitializeToken>, 
        _token_decimal: u8, 
        token_name: String, 
        token_symbol: String,
        token_uri: String
    ) -> Result<()> {

        msg!("Initializing the metadata_account");

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3{
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    mint_authority: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info()
                }
            ),
            DataV2 { name: token_name, symbol: token_symbol, uri: token_uri, seller_fee_basis_points: 0, creators: None, collection: None, uses: None },
            false, // This token isn't mutable
            true, // update authority is a signer
            None // Collection details
        )?;

        msg!("Token created");

        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(_token_decimal: u8)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account( 
        mut, 
        seeds = [b"metadata_account", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: This is an unsafe account
    pub metadata_account: UncheckedAccount<'info>, // This account is not initialized here because create_metadata_account_v3 will take care of it
    #[account(
        init, 
        payer = payer,
        mint::decimals = _token_decimal,
        mint::authority = payer
    )]
    pub mint_account: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}