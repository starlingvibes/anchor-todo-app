use anchor_lang::prelude::*;

declare_id!("HesbH8AsMBkxdgMRzfj1DWoqKWKcspZ8oVi2Y7ZL2D5u");

#[program]
pub mod todo_list {
    use super::*;
    // instructions
    pub fn initialize_task_list(ctx: Context<InitializeTaskList>) -> Result<()> {
        let task_list = &mut ctx.accounts.task_list;
        task_list.owner = *ctx.accounts.user.key;
        task_list.task_count = 0;
        Ok(())
    }

    pub fn add_task(ctx: Context<AddTask>, name: String) -> Result<()> {
        let task_list = &mut ctx.accounts.task_list;
        let task = &mut ctx.accounts.task;
        task.task_id = task_list.task_count;
        task.name = name;
        task.is_complete = false;
        task_list.task_count += 1;

        Ok(())
    }

    pub fn mark_task_complete(ctx: Context<MarkTaskComplete>) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let task_list = &mut ctx.accounts.task_list;

        require_eq!(task_list.owner, *ctx.accounts.user.key);
        task.is_complete = true;

        Ok(())
    }

    // edit name of the task
    pub fn edit_task_name(ctx: Context<EditTaskName>, _name: String) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let task_list = &mut ctx.accounts.task_list;

        require_eq!(task_list.owner, *ctx.accounts.user.key);
        task.name = _name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTaskList<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub task_list: Account<'info, TaskList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(mut)]
    pub task_list: Account<'info, TaskList>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"task", task_list.key().as_ref(), &[task_list.task_count as u8]],
        bump
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkTaskComplete<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub task_list: Account<'info, TaskList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditTaskName<'info> {
    #[account(mut)]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub task_list: Account<'info, TaskList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TaskList {
    pub owner: Pubkey,
    pub task_count: u32,
}
// account 'cos we're not doing any logic
#[account]
pub struct Task {
    pub task_id: u32,
    pub name: String,
    pub is_complete: bool,
}
