
use anchor_lang::prelude::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod capture_action {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let post = &mut ctx.accounts.post;
        post.complete_views = 0;
        post.likes = 0;
        post.shares = 0;
        post.multiple_views = 0;
        post.downloads = 0;
        post.comments = 0;
        post.posts = 0;              
        Ok(())
    }

    pub fn media_post(
        ctx: Context<MediaPost>,
        mediaobjectid: String,
        user_public_key: Pubkey
    ) -> ProgramResult {
        let post = &mut ctx.accounts.post; 

        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_ids.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserPostedMedia.into());
        }

        post.posts += 1;        

        post.mediaobject_ids.push(mediaobjectid);
        post.creator = user_public_key;

        Ok(())
    }

    pub fn media_complete_view(ctx: Context<MediaCompleteView>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;               

        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_complete_viewed.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserCompleteViewedMedia.into());
        }

        post.complete_views += 1;
        post.mediaobject_complete_viewed.push(mediaobjectid);

        Ok(())
    }

    pub fn media_like(ctx: Context<MediaLike>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;       
        
        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_liked.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserLikedMedia.into());
        }

        post.likes += 1;
        post.mediaobject_liked.push(mediaobjectid);

        Ok(())
    }

    pub fn media_share(ctx: Context<MediaShare>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;  
        
        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_shared.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserSharedMedia.into());
        }

        post.shares += 1;
        post.mediaobject_shared.push(mediaobjectid);

        Ok(())
    }

    pub fn media_multiple_view(ctx: Context<MediaMultipleView>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;  
        
        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_multiple_viewed.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserMultipleViewedMedia.into());
        }

        post.multiple_views += 1;
        post.mediaobject_multiple_viewed.push(mediaobjectid);

        Ok(())
    }

    pub fn media_download(ctx: Context<MediaDownload>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;   
        
        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_downloaded.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserDownloadedMedia.into());
        }

        post.downloads += 1;
        post.mediaobject_downloaded.push(mediaobjectid);

        Ok(())
    }

    pub fn media_comment(ctx: Context<MediaComment>, mediaobjectid: String) -> ProgramResult {
        let post = &mut ctx.accounts.post;      
        
        if mediaobjectid.chars().count() != 20 {
            return Err(Errors::MediaObjectInvalid.into())
        }

        let mut iter = post.mediaobject_commented.iter();
        if iter.any(|v| v == &mediaobjectid) {
            return Err(Errors::UserCommentedMedia.into());
        }

        post.comments += 1;
        post.mediaobject_commented.push(mediaobjectid);

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10008 )]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MediaPost<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaCompleteView<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaLike<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaShare<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaMultipleView<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaDownload<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[derive(Accounts)]
pub struct MediaComment<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>,
}

#[account] //An attribute for a data structure representing a Solana account.
#[derive(Default)]
pub struct Post {
    complete_views: u8,
    likes: u8,
    shares: u8,
    multiple_views: u8,
    downloads: u8,
    comments: u8,
    posts: u8,
    mediaobject_ids: Vec<String>,       
    mediaobject_liked: Vec<String>,
    mediaobject_downloaded: Vec<String>,
    mediaobject_complete_viewed: Vec<String>,
    mediaobject_shared: Vec<String>,
    mediaobject_multiple_viewed: Vec<String>,
    mediaobject_commented: Vec<String>,
    creator: Pubkey,
}

#[error]
pub enum Errors {   
    #[msg("Media object is invalid")]
    MediaObjectInvalid,

    #[msg("User has already posted that media object id")]
    UserPostedMedia,

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
