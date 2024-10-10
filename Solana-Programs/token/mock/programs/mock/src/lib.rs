use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, Mint, mint_to, TokenAccount, MintTo},
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3,
        CreateMasterEditionV3, CreateMetadataAccountsV3, mpl_token_metadata::types::DataV2,
        Metadata
    }
};

declare_id!("A2YnRqimpYk7KkW67ffMoNQtE9X3wLZJC8BcdYxbj3Sh");

#[program]
pub mod mock {
    use super::*;

    pub fn create_nft(ctx: Context<CreateNFTToken>, nft_name: String, nft_symbol: String, nft_uri: String) -> Result<()> {
        msg!("Minting token");

        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo { mint:ctx.accounts.mint_account.to_account_info(), 
                    to: ctx.accounts.associated_token_account.to_account_info(),
                     authority: ctx.accounts.payer.to_account_info() 
                    }
            ),
            1
        )?;

        msg!("creating metadata account");

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3{
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    mint_authority: ctx.accounts.payer.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                }
            ), 
            DataV2 { name: nft_name, symbol: nft_symbol, uri: nft_uri, seller_fee_basis_points: 0, creators: None, collection: None, uses: None },
            false,
            true, 
            None)?;

        msg!("Creating Master Edition account");

        create_master_edition_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3{
                    edition: ctx.accounts.edition_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                    mint_authority: ctx.accounts.payer.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                }
            )
        , None)?;// The second argument being none suggest that there isn't going to be any copies/editions of the specified NFT

        msg!("NFT minted");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNFTToken<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: This ain't safe
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub edition_account: UncheckedAccount<'info>,
    /// CHECK: This ain't safe either!
    #[account(
        mut,
        seeds = [b"metadata", token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition"],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init, 
        mint::decimals = 0,
        payer = payer,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key()
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init, 
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}
