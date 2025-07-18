use anchor_lang::prelude::*;

declare_id!("7fN8wFBskQw4z4cQwtQk2ESiLmFjL3Yxw1uK4GVHAN61");

#[program]
pub mod sol_dev {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let storage = &mut ctx.accounts.my_storage;
        storage.x = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 8,
        seeds = [],
        bump
    )]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
