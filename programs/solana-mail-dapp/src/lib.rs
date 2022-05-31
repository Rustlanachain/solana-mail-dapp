use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_mail_dapp {
    use super::*;

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let mail: &mut Account<Mail> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let reciever=&ctx.accounts.receiver;
        let clock: Clock = Clock::get().unwrap();

        mail.author = *author.key;
        mail.reciever =  *reciever.key;
        mail.subject = topic;
        mail.body = content;
        mail.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = 4000)]
    pub tweet: Account<'info, Mail>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    /// CHECK:
    pub system_program: AccountInfo<'info>,
}

// 1. Define the structure of the Tweet account.
#[account]
pub struct Mail {
    pub author: Pubkey,
    pub reciever: Pubkey,
    pub subject: String,
    pub body: String,
    pub timestamp: i64,
}
