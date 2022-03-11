use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;

use state::Unimoon;
use state::UserSacPair;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum UserAction {
    // action = point
    View = 1,
    Like = 2,
    Share = 3,
    Comment = 5,
    Download = 7,
}

#[program]
pub mod unimoon_base {
    use super::*;

    /// initialize
    pub fn initialize(ctx: Context<Initialize>, unimoon_account_bump: u8) -> Result<()> {
        let unimoon = &mut ctx.accounts.unimoon;
        unimoon.bump = unimoon_account_bump;
        unimoon.pairs = Vec::new();
        Ok(())
    }

    pub fn add_pair(ctx: Context<AddPair>, user: Pubkey, sac: u64) -> Result<()> {
        let pair = UserSacPair { user, sac };
        ctx.accounts.unimoon.pairs.push(pair);
        Ok(())
    }

    // /// update score of profile by user's action
    // pub fn do_post(ctx: Context<DoPost>, action: UserAction) -> ProgramResult {
    //     let post = &mut ctx.accounts.post;
    //     match action {
    //         UserAction::View => post.views += 1,
    //         UserAction::Like => post.likes += 1,
    //         UserAction::Share => post.shares += 1,
    //         UserAction::Comment => post.total_comments += 1,
    //         UserAction::Download => post.downloads += 1,
    //     };
    //     let post_creator = &mut ctx.accounts.post_creator;
    //     let post_creator_key = *post_creator.to_account_info().key;
    //     if post_creator_key != post.creator {
    //         return Err(ProgramError::InvalidAccountData)
    //     }
    //     post_creator.sac += action as u64;
    //     Ok(())
    // }
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(init, seeds = [b"unimoon_v0".as_ref(), user.key().as_ref()], bump, payer = user)]
    pub unimoon: Account<'info, Unimoon>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct DoPost<'info> {
//     // #[account(mut, has_one = authority)]
//     // pub user_profile: Account<'info, UserProfile>,
//     // pub authority: Signer<'info>,
//     #[account(mut)]
//     pub post: Account<'info, Post>,
//     #[account(mut)]
//     pub post_creator: Account<'info, UserProfile>,
// }

#[derive(Accounts)]
pub struct AddPair<'info> {
    #[account(mut)]
    pub unimoon: Account<'info, Unimoon>,
}