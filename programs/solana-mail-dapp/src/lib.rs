use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_mail_dapp {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        // if topic.chars().count() > 50 {
        //     // Return a error...
        //     return Err(ErrorCode::TopicTooLong.into())
        // }
    
        if content.chars().count() > 280 {
            // Return a error...
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = 4000)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK:
    pub system_program: AccountInfo<'info>,
}

// 1. Define the structure of the Tweet account.
#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}
