use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct  Participant {
      pub authority: Pubkey,
   pub  is_winner: bool,
   pub  bump: u8,
}