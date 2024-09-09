use anchor_lang::prelude::*;

declare_id!("2Vavk11C3yQQkEHxGE8zg62vFqsAZjZVJPrhzSe25NsZ");

#[program]
pub mod fuzzing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
