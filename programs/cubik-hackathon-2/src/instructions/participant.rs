use anchor_lang::prelude::*;
use crate::state::{
    Participant,
};
use anchor_lang::solana_program::{self,system_program};

#[derive(Accounts)]
#[instruction(create_key: Pubkey)]
pub struct TestMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

  
    #[account(
        init,
        payer = authority,
        space = 8+1+32+32,
        seeds = [b"hack",authority.key().as_ref(),create_key.as_ref()],
        bump,
    )]
    pub participant_account: Account<'info, Participant>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info,System>,

}


pub fn handler(ctx: Context<TestMint>,name:String,symbol:String,metadata_url:String,_counter: u16,create_key:Pubkey) -> Result<()> {
    
    let participant_account = &mut ctx.accounts.participant_account;
    participant_account.authority = ctx.accounts.authority.key();
    participant_account.is_winner = false;
    participant_account.bump = *ctx.bumps.get("participant_account").unwrap();

    msg!("NFT Minting");

    Ok(())
}