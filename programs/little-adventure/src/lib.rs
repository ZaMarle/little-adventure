use anchor_lang::prelude::*;

declare_id!("DS8NBLvfyvBFXnpg85pD6B3vwGn8p9cNW1YrgtyWH31d");

#[program]
pub mod little_adventure {
    use super::*;

    pub fn initialize(context: Context<Initialize>) -> Result<()> {
        context.accounts.new_game_data_account.player_position = 2;
        msg!("Game begins!");

        Ok(())
    }

    pub fn move_right(context: Context<MoveRight>) -> Result<()> {
        context.accounts.game_data_account.player_position += 1;
        // if context.accounts.game_data_account.player_position < 3 {
        //     context.accounts.game_data_account.player_position += 1;
        // } else {
        //     msg!("Already at the end!")
        // }

        Ok(())
    }

    pub fn move_left(context: Context<MoveLeft>) -> Result<()> {
        if context.accounts.game_data_account.player_position < 1 {
            context.accounts.game_data_account.player_position -= 1;
        } else {
            msg!("You are already at the start")
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"Level3"],
        bump,
        payer = signer,
        space = 8 + 8)]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameDataAccount {
    pub player_position: u8,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveLeft<'info> {
    pub game_data_account: Account<'info, GameDataAccount>,
}
