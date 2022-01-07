use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod binary_game {
    use super::*;
    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.price = 0.0;
        base_account.bet = 0;
        Ok(())
    }

    pub fn setBet(ctx: Context<SetBet>,data: f64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.price = f64;
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct SetBet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub price: f64,
    pub bet: u64,
}