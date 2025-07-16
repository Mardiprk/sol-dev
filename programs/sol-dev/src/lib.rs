use anchor_lang::prelude::*;

declare_id!("7fN8wFBskQw4z4cQwtQk2ESiLmFjL3Yxw1uK4GVHAN61");

#[program]
pub mod sol_dev {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
