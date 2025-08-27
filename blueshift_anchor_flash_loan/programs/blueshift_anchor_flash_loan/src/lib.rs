use anchor_lang::prelude::*;

declare_id!("Cq1EBT9b1yejknH4dLfLuaJ8X6db1KkstpTM3tP5hCDP");

#[program]
pub mod blueshift_anchor_flash_loan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
