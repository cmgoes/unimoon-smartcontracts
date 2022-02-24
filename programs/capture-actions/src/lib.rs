use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Mint, MintTo, SetAuthority, TokenAccount};
use spl_token::instruction::AuthorityType;

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
        user_profile.sac = 0;
        Ok(())
    }

    /// write a post
    pub fn write_post(ctx: Context<WritePost>) -> ProgramResult {
        // First check that this token didn't mint anything yet
        if ctx.accounts.mint.supply != 0 || ctx.accounts.mint.decimals != 0 {
            return Err(ProgramError::InvalidArgument)
        };

        let user_profile = &mut ctx.accounts.user_profile;
        let post = &mut ctx.accounts.post;
        // init state of post
        post.views = 0;
        post.likes = 0;
        post.shares = 0;
        post.total_comments = 0;
        post.downloads = 0;
        post.sac = 0;
        post.creator = *user_profile.to_account_info().key;
        let token = &mut ctx.accounts.token;
        post.token = *token.to_account_info().key;

        // Mint NFT and delete mint authority
        token::mint_to(ctx.accounts.mint_to(), 1)?;
        token::set_authority(
            ctx.accounts.null_authority(),
            AuthorityType::MintTokens,
            None,
        )?;
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
        post_creator.sac += action as u64;
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
    #[account(init, payer = authority, space = 8 + 48 + 64)]
    pub post: Account<'info, Post>,
    pub system_program: Program<'info, System>,

    #[account(mut, constraint = mint.mint_authority == COption::Some(*authority.key))]
    pub mint: Account<'info, Mint>,
    #[account(mut, has_one = mint)]
    pub token: Account<'info, TokenAccount>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}

impl<'info> WritePost<'info> {
    fn mint_to(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let account = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.token.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        let program = self.token_program.to_account_info();
        CpiContext::new(program, account)
    }

    fn null_authority(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let account = SetAuthority {
            current_authority: self.authority.to_account_info().clone(),
            account_or_mint: self.mint.to_account_info().clone(),
        };
        let program = self.token_program.clone();
        CpiContext::new(program, account)
    }
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
