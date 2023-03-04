use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod state;
use crate::{constant::*, error::*, state::*};

declare_id!("A5jbvgyHtr1bbcPazdYPjS2UXfBTxbA6p1FmQfVvVoWq");

#[program]
pub mod main {
    use super::*;

    pub fn init(ctx: Context<InitUser>) -> Result<()> {
        // Initialize user profile with default data
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.task_last = 0;
        user_profile.task_count = 0;

        Ok(())
    }

    pub fn create_task(ctx: Context<CreateTask>, content: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let user = &mut ctx.accounts.user_profile;

        task.authority = ctx.accounts.authority.key();
        task.idx = user.task_last;
        task.content = content.clone();
        task.status = TODO_STATUS_TODO;

        user.task_last += 1;
        user.task_count += 1;

        Ok(())
    }

    pub fn delete_task(ctx: Context<DeleteTask>, idx: u8) -> Result<()> {
        let user = &mut ctx.accounts.user_profile;

        if user.task_count > 0 {
            user.task_count -= 1;
        }

        Ok(())
    }

    pub fn set_status_task(ctx: Context<SetStatusTask>, idx: u8, status: u8) -> Result<()> {
        let task = &mut ctx.accounts.task;

        if status == TODO_STATUS_START {
            require!(task.status == TODO_STATUS_TODO, LibraError::NotAllowed);
        } else if status == TODO_STATUS_DONE || status == TODO_STATUS_ABANDON {
            require!(task.status == TODO_STATUS_START, LibraError::NotAllowed);
        }

        task.status = status;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<User>(),
    )]
    pub user_profile: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateTask<'info> {
    #[account(
        mut,
        seeds = [USER_TAG],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, User>>,

    #[account(
        init,
        seeds = [TODO_TAG, &[user_profile.task_count as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<Task>() + 8,
    )]
    pub task: Box<Account<'info, Task>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idx: u8)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        seeds = [USER_TAG],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, User>>,

    #[account(
        mut,
        close = authority,
        seeds = [TODO_TAG, &[idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub task: Box<Account<'info, Task>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idx: u8)]
pub struct SetStatusTask<'info> {
    #[account(
        mut,
        seeds = [USER_TAG],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, User>>,

    #[account(
        mut,
        seeds = [TODO_TAG, &[idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub task: Box<Account<'info, Task>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
