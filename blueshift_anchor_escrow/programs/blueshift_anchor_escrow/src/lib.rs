use anchor_lang::prelude::*;

declare_id!("89hAXJHytGv8kmdi6iRVbLhoA2diUNYaYFy19JF84qZF");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
