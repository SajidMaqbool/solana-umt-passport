use anchor_lang::prelude::*;

declare_id!("CLEhaRz7CnTWEQNaAonRktJDyVP3YsCMQ8GEPnc2BZ5k");

#[program]
pub mod solana_passport {
    use super::*;

    pub fn create_passport(ctx: Context<CreatePassport>, name: String, student_id: String) -> Result<()> {
        let passport = &mut ctx.accounts.passport;
        passport.name = name;
        passport.student_id = student_id;
        passport.authority = *ctx.accounts.user.key;
        Ok(())
    }
}

#[account]
pub struct Passport {
    pub name: String,        // Student Name
    pub student_id: String,  // Roll Number
    pub authority: Pubkey,   // Wallet Address
}

#[derive(Accounts)]
pub struct CreatePassport<'info> {
    #[account(init, payer = user, space = 8 + 40 + 20 + 32)]
    pub passport: Account<'info, Passport>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
