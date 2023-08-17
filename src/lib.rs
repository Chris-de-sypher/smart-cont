use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
#![allow(unused)]

declare_id!("6tFq6HLfSU4ZagpGuXueU1Z6qPRy2DgK3N2og3i7yvPm");

#[program]
mod kallo_view_registry {
    use super::*;

    pub fn post_review(ctx: Context<PostReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
        let review_hash = hash(&ctx.accounts.author.key.to_bytes());

        if ctx.accounts.reviews.contains_key(&review_hash) {
            return Err(ErrorCode::AlreadyReviewed.into());
        }

        ctx.accounts.reviews.insert(&review_hash, &true)?;

        ctx.accounts.events.review_posted(location_id, review_id)?;

        Ok(())
    }

    pub fn comment_review(ctx: Context<CommentReview>, location_id: [u8; 32], review_id: [u8; 32], comment_id: [u8; 32]) -> ProgramResult {
        ctx.accounts.events.review_commented(location_id, review_id, comment_id)?;

        Ok(())
    }

    pub fn upvote_review(ctx: Context<UpvoteReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
        let voter = &ctx.accounts.voter.key;
        ctx.accounts.voted.ensure_voter_has_not_voted(review_id, voter)?;

        ctx.accounts.events.review_upvoted(location_id, review_id)?;

        Ok(())
    }

    pub fn downvote_review(ctx: Context<DownvoteReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
        let voter = &ctx.accounts.voter.key;
        ctx.accounts.voted.ensure_voter_has_not_voted(review_id, voter)?;

        ctx.accounts.events.review_downvoted(location_id, review_id)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostReview<'info> {
    #[account(init)]
    pub reviews: Account<'info, Map<Hash, bool>>,
    pub author: AccountInfo<'info>,
    pub events: Event<'info>,
}

#[derive(Accounts)]
pub struct CommentReview<'info> {
    pub events: Event<'info>,
}

#[derive(Accounts)]
pub struct UpvoteReview<'info> {
    pub voter: AccountInfo<'info>,
    pub voted: Voted<'info>,
    pub events: Event<'info>,
}

#[derive(Accounts)]
pub struct DownvoteReview<'info> {
    pub voter: AccountInfo<'info>,
    pub voted: Voted<'info>,
    pub events: Event<'info>,
}

#[derive(Accounts)]
pub struct Voted<'info> {
    #[account(init)]
    pub voted: Map<Hash, Map<AccountId, bool>>,
    pub system_program: AccountInfo<'info>,
}

#[event]
pub struct ReviewPosted {
    pub location_id: [u8; 32],
    pub review_id: [u8; 32],
    pub author: AccountId,
}

#[event]
pub struct ReviewUpvoted {
    pub location_id: [u8; 32],
    pub review_id: [u8; 32],
}

#[event]
pub struct ReviewDownvoted {
    pub location_id: [u8; 32],
    pub review_id: [u8; 32],
}

#[event]
pub struct ReviewCommented {
    pub location_id: [u8; 32],
    pub review_id: [u8; 32],
    pub comment_id: [u8; 32],
}

#[error]
pub enum ErrorCode {
    #[msg("Review already voted")]
    AlreadyVoted,
    #[msg("Review already reviewed")]
    AlreadyReviewed,
}

impl<'info> Voted<'info> {
    pub fn ensure_voter_has_not_voted(&self, review_id: [u8; 32], voter: &AccountId) -> ProgramResult {
        if self.voted.contains_key(&review_id) {
            if self.voted[&review_id].contains_key(voter) && self.voted[&review_id][voter] {
                return Err(ErrorCode::AlreadyVoted.into());
            }
        }
        Ok(())
    }
}
