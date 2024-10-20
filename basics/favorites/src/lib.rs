use anchor_lang::prelude::*;

declare_id!("4KKxP6UrnmuHMx5dHHvbVTzTMRDcmSog9Nu6LumnaWdh");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<SetFavorites>, number: u64, favorite_game: String, favorite_players: Vec<String>) -> Result<()> {
        let user = ctx.accounts.user.key;

        msg!("{user} admires {favorite_game}, and like {favorite_players:?}");

        ctx.accounts.fav.set_inner(Favorites{
            number, 
            favorite_game,
            favorite_players
        });

        Ok(())
    }
}


#[account]
//InitSpace implements Space trait on the following struct
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(20)]
    pub favorite_game: String,
    #[max_len(5, 10)]
    pub favorite_players: Vec<String>
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account[mut]]
    pub user: Signer<'info>,
    #[account[
        init_if_needed,
        seeds= [b"favorites", user.key().as_ref()],
        bump,
        space= 8 + Favorites::INIT_SPACE,
        payer= user
    ]]
    pub fav: Account<'info, Favorites>,
    pub system_program: Program<'info, System>
}