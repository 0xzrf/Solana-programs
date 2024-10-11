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

declare_id!("7PGhrDKEukToPT3EPawUGnV1HdkKnqBpY9vHhfKj575n");

#[program]
pub mod mock {
    use super::*;

    
}


