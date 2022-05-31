use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("83nKp3FskjAeCLwNWnrB6a4SStFC7fpi93LUAYsBwnCA");

#[program]
pub mod solana_mail_dapp {
    use super::*;

    pub fn send_mail(ctx: Context<SendMail>, topic: String, content: String) -> Result<()> {
        let mail: &mut Account<Mail> = &mut ctx.accounts.mail;
        let sender: &Signer = &ctx.accounts.sender;
        let reciever=&ctx.accounts.receiver;
        let clock: Clock = Clock::get().unwrap();

        mail.sender = *sender.key;
        mail.reciever =  *reciever.key;
        mail.subject = topic;
        mail.body = content;
        mail.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendMail<'info> {
    #[account(init, payer = sender, space = 4000)]
    pub mail: Account<'info, Mail>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    /// CHECK:
    pub system_program: AccountInfo<'info>,
}

// 1. Define the structure of the Mail.
#[account]
pub struct Mail {
    pub sender: Pubkey,
    pub reciever: Pubkey,
    pub subject: String,
    pub body: String,
    pub timestamp: i64,
}
