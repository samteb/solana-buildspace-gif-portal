use anchor_lang::prelude::*;

declare_id!("D4mKXftPNrDZzAN79DTBMQz1xL3xL4DkGGu4mCYnnrUc");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }


    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes: 0
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn up_vote(ctx: Context<UpdateGif>, index: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let i = index as usize;
        if i < base_account.gif_list.len() {
            let mut item = &mut base_account.gif_list[i];
            item.votes += 1;
        }

        Ok(())
    }

    pub fn down_vote(ctx: Context<UpdateGif>, index: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let i = index as usize;
        if i < base_account.gif_list.len() {
            let mut item = &mut base_account.gif_list[i];
            item.votes -= 1;
        }

        Ok(())
    }
}

#[error]
pub enum Err {
    #[msg("No item with that url found")]
    NoItemFound,
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpdateGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: i64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>
}
