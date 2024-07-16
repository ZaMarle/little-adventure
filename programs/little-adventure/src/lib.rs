use anchor_lang::prelude::*;

declare_id!("2uuwsb2JYLEFjqBUizwz2FwhzPhBSDRaXk2P1FfG6aNr");

#[program]
pub mod little_adventure {
    use super::*;

    pub fn initialize(context: Context<Initialize>) -> Result<()> {
        context.accounts.new_game_data_account.player_position = (0, 0, 0);
        msg!("Game begins!");

        Ok(())
    }

    pub fn move_player(
        context: Context<MovePlayer>,
        next_position: (u8, u8, u8),
    ) -> Result<()> {
        context.accounts.game_data_account.player_position = next_position;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"Level4"],
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
    pub player_position: (u8, u8, u8),
}

#[derive(Accounts)]
pub struct MovePlayer<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}
