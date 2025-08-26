use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 0)]
pub struct Escrow {
    pub seeds: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}
