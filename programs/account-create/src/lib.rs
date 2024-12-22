#![allow(clippy::result_large_err)]

use anchor_lang::accounts::signer;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};

declare_id!("FAq6cjgw4gtfaGARWbxE95tYFA5wQUAkr1fXayepJKJu");

#[program]
pub mod account_create {
    use super::*;

    pub fn create_system_account(ctx: Context<CreateSystemAccount>) -> Result<()> {
        msg!("program invoked. creating account .. .");
        msg!(
            "new pubkey will be {}",
            &ctx.accounts.new_account.key().toString()
        );

        let lamport = (Rent::get().minimum_balance(0));

        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                createAccount {
                    from: ctx.accounts.payer.to_account_info(), // from pubkey
                    to: ctx.accounts.new_account.to_account_info(), // to pubkey
                },
            ),
            lamport,
            0,
            &ctx.accounts.system_program.key(),
        )?;

        msg!("account created successfully")
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CreateSystemAccount<'info>{
    #[account(mut)]
    pub payer: signer<'info>

    #[account(mut)]
    pub new_account:Signer<'info>,
    pub system_program: Program<'info, System>,
}
