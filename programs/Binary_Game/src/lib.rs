use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod binary_game {
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        Ok(())
    }

    pub fn place_Bet(ctx: Context<SetBet>,price: f64, stake_bal: u64, user: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let pool_wallet = &mut ctx.accounts.pool_wallet;

        // TODO: check prediction is one of possible options 
        // TODO: Add payment to this function 
        // TODO: make sure today is before the close date. 

        //Build the struct.
        let bet_item = BetStruct {
            price: price,
            stake_bal: stake_bal,
            user: user,
        }

        base_account.current_bet = bet_item;

        let sb = stake_bal as u64;
        let account_lamports = **pool_wallet.to_account_info().lamports.borrow();
        let transfer_amount = sb.checked_sub(account_lamports).ok_or(0)?;

        if price > 0 {
            invoke( 
                &transfer(
                    ctx.accounts.user.to_account_info().key,
                    pool_wallet.to_account_info().key,
                    transfer_amount,
                ),
                &[
                    ctx.accounts.user.to_account_info(),
                    pool_wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],    
            )?;
        }

        Ok(())
    }

    pub fn compare_Bet(ctx: Context<CompareBet>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..2));

        match random_number {
            0 => base_account.bool_winner = false,
            1 => base_account.base_winner = true,
        }

    }

    pub fn result_Bet(ctx: Context<SetBet>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let pool_wallet = &mut ctx.accounts.pool_wallet;

        let bet_item = base_account.current_bet;

        let sb = bet_item.stake_bal as u64;
        let account_lamports = **pool_wallet.to_account_info().lamports.borrow();
        let transfer_amount = 2 * sb.checked_sub(account_lamports).ok_or(0)?;

        if base_account.bool_winner == true {
            invoke( 
                &transfer(
                    pool_wallet.to_account_info().key,
                    ctx.accounts.user.to_account_info().key,
                    transfer_amount,
                ),
                &[
                    pool_wallet.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.system_program.to_account_info()
                ],    
            )?;
        }

        Ok(())
    }

}
   
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
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
    #[account(init, seeds=[pool_name.as_bytes(),b"pool_wallet"], space=9000, bump = 5, payer=user)]
    pub pool_wallet: Account<'info, PoolWallet>,
    #[account(mut)]
    pub user: Signer<info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CompareBet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BetStruct {
    pub stake_bal: f32,
    pub prediction: u64,
    pub user: String,
}

#[account]
pub struct BaseAccount {
    pub bool_winner: bool,
    pub current_bet: BetStruct,
}

#[account]
pub struct PoolWallet{
    pub balance: u64 
} 