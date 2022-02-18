use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod state;

use state::UserProfile;

#[program]
pub mod capture_actions {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>) -> ProgramResult {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = *ctx.accounts.user.key;
        user_profile.score = 0;
        Ok(())
    }

    pub fn upload_content(ctx: Context<UploadContent>) -> ProgramResult {
        let user_profile = &mut ctx.accounts.user_profile;
        // TODO: Mint NFT, update user's score per action.
        
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
pub struct UploadContent<'info> {
    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}
