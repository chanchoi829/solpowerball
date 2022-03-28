use anchor_lang::prelude::*;

declare_id!("8rdLvCDitZ7CMXAWZpVBr3pjxXY5Y46acSRzReBP5VMp");

#[program]
pub mod solpowerball {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
