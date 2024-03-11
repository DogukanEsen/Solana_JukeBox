use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("bYj1YXjVRxqte3evzi8eLJ6PrdHYGDfoJsBCk22bi1u");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        
        let escrow = &mut ctx.accounts.escrow;
        escrow.from = ctx.accounts.from.key();
        escrow.to = ctx.accounts.to.key();
        escrow.amount = amount;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(
        init,
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>() + 10
    )]
    pub escrow: Account<'info,EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount{
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}