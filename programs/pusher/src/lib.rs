use anchor_lang::prelude::*;

declare_id!("CUiE3zokcqrzQxDveCjL7x4edhUZuGDbSyCVrxnzyJNu");

#[program]
pub mod pusher {
    use super::*;

    // pub fn pushIntoBlockchain(ctx: Context<Initialize>, data: String, sno: u64) -> Result<()> {
    //     &ctx.accounts.pushIntoBlockchain.data = data;
    //     &ctx.accounts.pushIntoBlockchain.sno = sno;
    //     Ok(())
    // }
    pub fn addintoCoutner(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter value: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    #[account(init_if_needed, payer = user, space = 8 + 16, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    // pub pushIntoBlockchain: Account<'info, PushIntoBlockchain>,
}
#[account]
pub struct Counter {
    pub count: u64,
}
// #[account]
// pub struct PushIntoBlockchain {
//     pub data: String,
//     pub sno: u64,
// }
