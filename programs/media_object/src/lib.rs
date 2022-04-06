
use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod media_object {
  use super::*;
  pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;
    actobject.complete_views = 0;
    actobject.likes = 0;
    actobject.shares = 0;
    actobject.multiple_views = 0;
    actobject.downloads = 0;
    actobject.comments = 0;
    actobject.mediaobject_id = ("").to_string();              
    Ok(())
  }

  pub fn act_media_post(
    ctx: Context<ActMediaPost>,
    mediaobjectid: String,
    user_public_key: Pubkey
  ) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject; 

    if mediaobjectid.chars().count() != 20 {
      return Err(Errors::MediaObjectInvalid.into())
    }        

    actobject.mediaobject_id = mediaobjectid;  
    actobject.complete_views = 0;
    actobject.likes = 0;
    actobject.shares = 0;
    actobject.multiple_views = 0;
    actobject.downloads = 0;
    actobject.comments = 0;  
    actobject.creator = user_public_key;
    Ok(())
  }

  pub fn act_media_complete_view(ctx: Context<ActMediaCompleteView>, user_complete_view_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;               

    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_complete_viewed.iter();
    if iter.any(|&v| v == user_complete_view_key) {
      return Err(Errors::UserCompleteViewedMedia.into());
    }

    actobject.complete_views += 1;
    actobject.user_complete_viewed.push(user_complete_view_key);

    Ok(())
  }

  pub fn act_media_like(ctx: Context<ActMediaLike>, user_like_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;       
    
    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_liked.iter();
    if iter.any(|&v| v == user_like_key) {
      return Err(Errors::UserLikedMedia.into());
    }

    actobject.likes += 1;
    actobject.user_liked.push(user_like_key);

    Ok(())
  }

  pub fn act_media_share(ctx: Context<ActMediaShare>, user_share_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;  
    
    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_shared.iter();
    if iter.any(|&v| v == user_share_key) {
      return Err(Errors::UserSharedMedia.into());
    }

    actobject.shares += 1;
    actobject.user_shared.push(user_share_key);

    Ok(())
  }

  pub fn act_media_multiple_view(ctx: Context<ActMediaMultipleView>, user_multiple_view_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;  
    
    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_multiple_viewed.iter();
    if iter.any(|&v| v == user_multiple_view_key) {
      return Err(Errors::UserMultipleViewedMedia.into());
    }

    actobject.multiple_views += 1;
    actobject.user_multiple_viewed.push(user_multiple_view_key);

    Ok(())
  }

  pub fn act_media_download(ctx: Context<ActMediaDownload>, user_download_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;   
    
    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_downloaded.iter();
    if iter.any(|&v| v == user_download_key) {
      return Err(Errors::UserDownloadedMedia.into());
    }

    actobject.downloads += 1;
    actobject.user_downloaded.push(user_download_key);

    Ok(())
  }

  pub fn act_media_comment(ctx: Context<ActMediaComment>, user_comment_key: Pubkey) -> ProgramResult {
    let actobject = &mut ctx.accounts.actobject;      
    
    if actobject.mediaobject_id.trim().is_empty() {
      return Err(Errors::MediaObjectInvalid.into())
    }

    let mut iter = actobject.user_commented.iter();
    if iter.any(|&v| v == user_comment_key) {
      return Err(Errors::UserCommentedMedia.into());
    }

    actobject.comments += 1;
    actobject.user_commented.push(user_comment_key);

    Ok(())
  }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 10008 )]
  pub actobject: Account<'info, ActObject>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActMediaPost<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaCompleteView<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaLike<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaShare<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaMultipleView<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaDownload<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[derive(Accounts)]
pub struct ActMediaComment<'info> {
  #[account(mut)]
  pub post: Account<'info, ActObject>,
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct ActObject {
  complete_views: u8,
  likes: u8,
  shares: u8,
  multiple_views: u8,
  downloads: u8,
  comments: u8,    
  mediaobject_id: String,       
  user_liked: Vec<String>,
  user_downloaded: Vec<String>,
  user_complete_viewed: Vec<String>,
  user_shared: Vec<String>,
  user_multiple_viewed: Vec<String>,
  user_commented: Vec<String>,
  creator: Pubkey,
}

#[error]
pub enum Errors {   
  #[msg("Media object is invalid")]
  MediaObjectInvalid,   

  #[msg("User has already viewed media object completely")]
  UserCompleteViewedMedia,

  #[msg("User has already liked media object")]
  UserLikedMedia,

  #[msg("User has already shared media object")]
  UserSharedMedia,

  #[msg("User has already viewed media object multiple times")]
  UserMultipleViewedMedia,

  #[msg("User has already downloaded media object")]
  UserDownloadedMedia,

  #[msg("User has already commented media object")]
  UserCommentedMedia,    
}
