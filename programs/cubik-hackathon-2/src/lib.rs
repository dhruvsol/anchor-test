use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;
declare_id!("BAsAdkqgegzZ5ED7CZcFsC1CxHhg4S2ESuUiK2eESwLY");

#[program]
pub mod cubik_hackathon_2 {
    use super::*;

   pub fn  participant(ctx: Context<TestMint>,name:String,symbol:String,metadata_url:String,counter: u16,create_key:Pubkey) -> Result<()> {
        participant::handler(ctx,name,symbol,metadata_url,counter,create_key);
         Ok(())
    } 
}


