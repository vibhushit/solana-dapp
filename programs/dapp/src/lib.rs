use anchor_lang::prelude::*;

declare_id!("ifQ4txp3ZLwdqgERWGRDna67Gs1JesqHQnFtGoad7H7");

#[program]
pub mod dapp {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        require!(init_message.len() <= 240, CustomError::MessageTooLong);
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, num1 : i64, num2 : i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn sub(ctx: Context<Subtraction>, num1 : i64, num2 : i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1 : i64, num2 : i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn divide(ctx: Context<Division> , num1 : i64, num2 : i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1/num2;
        calculator.remainder = num1%num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 4 + 240 + 8 + 8)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator : Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator : Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator : Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator : Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

#[error_code]
pub enum CustomError {
    #[msg("The provided message exceeds the maximum length.")]
    MessageTooLong,
}
