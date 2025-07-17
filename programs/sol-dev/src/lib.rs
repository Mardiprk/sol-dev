use anchor_lang::prelude::*;

declare_id!("7fN8wFBskQw4z4cQwtQk2ESiLmFjL3Yxw1uK4GVHAN61");

#[program]
pub mod sol_dev {
    use super::*;

    pub fn check_balance(ctx: Context<CheckBalance>) -> Result<u64> {
        let balance = ctx.accounts.target.lamports();
        msg!("Wallet Balance (lamports): {}", balance);
        Ok(balance)
    }
}

#[derive(Accounts)]
pub struct CheckBalance<'info> {
    /// CHECK: This account is only used to read the balance, no validation needed
    pub target: AccountInfo<'info>,
}