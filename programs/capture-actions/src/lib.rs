use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;

use state::UserProfile;
use state::Post;

#[derive(AnchorDeserialize)]
#[derive(AnchorSerialize)]
pub enum UserAction {
    VIEW,
    LIKE,
    SHARE,
    COMMENT,
    DOWNLOAD,
}

#[program]
pub mod capture_actions {
    use super::*;

    /// create a user's profile
    pub fn create_profile(ctx: Context<CreateProfile>) -> ProgramResult {
        let user_profile = &mut ctx.accounts.user_profile;
        // init profile
        user_profile.authority = *ctx.accounts.user.key;
        user_profile.score = 0;
        Ok(())
    }

    /// write a post
    pub fn write_post(ctx: Context<WritePost>) -> ProgramResult {
        let user_profile = &mut ctx.accounts.user_profile;
        let post = &mut ctx.accounts.post;
        // init state of post
        post.views = 0;
        post.likes = 0;
        post.shares = 0;
        post.total_comments = 0;
        post.downloads = 0;
        post.creator = *user_profile.to_account_info().key;

        Ok(())
    }

    /// update score of profile by user's action
    pub fn do_post(ctx: Context<DoPost>, action: UserAction) -> ProgramResult {
        let post = &mut ctx.accounts.post;
        match action {
            UserAction::VIEW => post.views += 1,
            UserAction::LIKE => post.likes += 1,
            UserAction::SHARE => post.shares += 1,
            UserAction::COMMENT => post.total_comments += 1,
            UserAction::DOWNLOAD => post.downloads += 1,
        };
        let post_creator = &mut ctx.accounts.post_creator;
        let post_creator_key = *post_creator.to_account_info().key;
        if post_creator_key != post.creator {
            return Err(ProgramError::InvalidAccountData)
        }
        post_creator.score += action as u128;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(init, payer = user, space = 8 + 16 + 32)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WritePost<'info> {
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + 5 + 32)]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DoPost<'info> {
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub post_creator: Account<'info, UserProfile>,
}
