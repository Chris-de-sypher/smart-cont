use anchor_lang::prelude::*;

#[program]
mod kallo_view_registry_test {
    use super::*;

    pub fn set_up(ctx: Context<SetUp>) -> ProgramResult {
        let registry = KalloViewRegistry::new(ctx.accounts.registry.to_account_info().clone());
        registry.initialize()?;
        Ok(())
    }

    pub fn test_post_review(ctx: Context<TestPostReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
        // Assumes and emits
        ctx.emit_event(ReviewPosted {
            location_id,
            review_id,
            author: *ctx.accounts.user.key,
        });

        ctx.accounts.registry.post_review(location_id, review_id)?;
        Ok(())
    }


    pub fn test_upvote_review(ctx: Context<TestUpvoteReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
    // Assumes and emits
    ctx.emit_event(ReviewUpvoted {
        location_id,
        review_id,
        voter: *ctx.accounts.user.key,
    });

    ctx.accounts.registry.upvote_review(location_id, review_id)?;
    Ok(())
    }

    pub fn test_downvote_review(ctx: Context<TestDownvoteReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
    // Assumes and emits
    ctx.emit_event(ReviewDownvoted {
        location_id,
        review_id,
        voter: *ctx.accounts.user.key,
    });

    ctx.accounts.registry.downvote_review(location_id, review_id)?;
    Ok(())
    }

    pub fn test_comment_review(ctx: Context<TestCommentReview>, location_id: [u8; 32], review_id: [u8; 32], comment_id: [u8; 32]) -> ProgramResult {
    // Assumes and emits
    ctx.emit_event(ReviewCommented {
        location_id,
        review_id,
        comment_id,
        author: *ctx.accounts.user.key,
    });

    ctx.accounts.registry.comment_review(location_id, review_id, comment_id)?;
    Ok(())
    }

    // 

    #[derive(Accounts)]
    pub struct SetUp<'info> {
        #[account(init, payer = user, space = 8 + 8 + 32 + 32)]
        pub registry: Account<'info, KalloViewRegistry>,
        #[account(mut, signer)]
        pub user: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct TestPostReview<'info> {
        #[account(mut)]
        pub registry: Box<Account<'info, KalloViewRegistry>>,
        #[account(signer)]
        pub user: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct TestUpvote<'info> {
        #[account(mut)]
        pub registry: Box<Account<'info, KalloViewRegistry>>,
        #[account(signer)]
        pub user: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct TestDownvote<'info> {
        #[account(mut)]
        pub registry: Box<Account<'info, KalloViewRegistry>>,
        #[account(signer)]
        pub user: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct TestComment<'info> {
        #[account(mut)]
        pub registry: Box<Account<'info, KalloViewRegistry>>,
        #[account(signer)]
        pub user: AccountInfo<'info>,
        pub system_program: AccountInfo<'info>,
    }


    #[account]
    pub struct KalloViewRegistry {
        pub reviews: Vec<Review>,
    }

    impl KalloViewRegistry {
        pub fn new<'info>(registry: Account<'info, KalloViewRegistry>) -> Self {
            KalloViewRegistry {
                reviews: Vec::new(),
            }
        }

        pub fn initialize(&mut self) -> ProgramResult {
            Ok(())
        }

        pub fn post_review(&mut self, ctx: Context<TestPostReview>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
            // Verify that the user is authorized to post a review
            if ctx.accounts.user.key != &self.author {
                return Err(ErrorCode::Unauthorized.into());
            }

            // Check if the review already exists
            let review_exists = self.reviews.contains_key(&location_id, &review_id);
            if review_exists {
                return Err(ErrorCode::ReviewAlreadyExists.into());
            }

            // Create a new review and store it
            let new_review = Review {
                author: *ctx.accounts.user.key,
                location_id,
                review_id,
                upvotes: 0,
                downvotes: 0,
                comments: Vec::new(),
            };

            self.reviews.try_insert(&location_id, &review_id, new_review)?;

            // Emit the ReviewPosted event
            ctx.emit_event(ReviewPosted {
                location_id,
                review_id,
                author: *ctx.accounts.user.key,
            });

            Ok(())
        }
        pub fn upvote_review(&mut self, ctx: Context<TestUpvote>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
            // Verify that the user is authorized to upvote
            if ctx.accounts.user.key != &self.author {
                return Err(ErrorCode::Unauthorized.into());
            }

            // Check if the review exists
            let review_exists = self.votes.contains_key(&location_id, &review_id);
            if !review_exists {
                return Err(ErrorCode::ReviewNotFound.into());
            }

            // Check if the user has already upvoted this review
            let has_upvoted = self.votes.get(&location_id, &review_id, &ctx.accounts.user.key);
            if has_upvoted.unwrap_or_default() {
                return Err(ErrorCode::AlreadyUpvoted.into());
            }

            // Mark the user as having upvoted this review
            self.votes
                .try_mutate(&location_id, &review_id, |voted_users| {
                    voted_users.insert(&ctx.accounts.user.key, &true);
                    Ok(())
                })?;

            // Emit the ReviewUpvoted event
            ctx.emit_event(ReviewUpvoted {
                location_id,
                review_id,
                author: *ctx.accounts.user.key,
            });

            Ok(())
        }
            pub fn downvote_review(&mut self, ctx: Context<TestDownvote>, location_id: [u8; 32], review_id: [u8; 32]) -> ProgramResult {
                // Verify that the user is authorized to downvote
                if ctx.accounts.user.key != &self.author {
                    return Err(ErrorCode::Unauthorized.into());
                }

                // Check if the review exists
                let review_exists = self.votes.contains_key(&location_id, &review_id);
                if !review_exists {
                    return Err(ErrorCode::ReviewNotFound.into());
                }

                // Check if the user has already downvoted this review
                let has_downvoted = self.votes.get(&location_id, &review_id, &ctx.accounts.user.key);
                if has_downvoted.unwrap_or_default() {
                    return Err(ErrorCode::AlreadyDownvoted.into());
                }

                // Mark the user as having downvoted this review
                self.votes
                    .try_mutate(&location_id, &review_id, |voted_users| {
                        voted_users.insert(&ctx.accounts.user.key, &true);
                        Ok(())
                    })?;

                // Emit the ReviewDownvoted event
                ctx.emit_event(ReviewDownvoted {
                    location_id,
                    review_id,
                    author: *ctx.accounts.user.key,
                });

                Ok(())
            }
                pub fn comment_review(&mut self, ctx: Context<TestComment>, location_id: [u8; 32], review_id: [u8; 32], comment_id: [u8; 32]) -> ProgramResult {
                    // Verify that the user is authorized to comment
                    if ctx.accounts.user.key != &self.author {
                        return Err(ErrorCode::Unauthorized.into());
                    }

                    // Check if the review exists
                    let review_exists = self.reviews.contains_key(&location_id, &review_id);
                    if !review_exists {
                        return Err(ErrorCode::ReviewNotFound.into());
                    }

                    // Add the comment to the review's comments
                    self.reviews.try_mutate(&location_id, &review_id, |review| {
                        review.comments.push(comment_id);
                        Ok(())
                    })?;

                    // Emit the ReviewCommented event
                    ctx.emit_event(ReviewCommented {
                        location_id,
                        review_id,
                        author: *ctx.accounts.user.key,
                    });

                    Ok(())
                }
    }
}