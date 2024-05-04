use anchor_lang::prelude::*;

declare_id!("G8ixXT7P656eaFqq9GXtHT1ut54hs3g1e1PoU7zPs84m");

#[program]
pub mod realloc {
    use anchor_lang::Discriminator;

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let data = &mut _ctx.accounts.data;
        data.field1 = 1;
        data.field2 = 2;
        data.field3 = 3;
        data.field4 = vec![1, 2, 3, 4];
        Ok(())
    }

    pub fn overwrite_discriminator(ctx: Context<ChangeDiscriminator>) -> Result<()> {
        let new_discriminator = NewData::discriminator();

        let new_state = NewData {
            field1: ctx.accounts.data.field1,
            field2: ctx.accounts.data.field2,
            field3: ctx.accounts.data.field3,
            field4: ctx.accounts.data.field4.clone(),
            field5: 5,
        };

        let account_info = ctx.accounts.data.to_account_info();
        let mut data = &mut *account_info.try_borrow_mut_data()?;
        let write_data = (new_discriminator, new_state);
        write_data.serialize(&mut data)?;
        Ok(())
    }

    pub fn realloc(_ctx: Context<Realloc>) -> Result<()> {
        let data = &mut _ctx.accounts.data;
        data.field4.push(5);
        data.field5 = 6;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = 1000,
        payer = signer,
        seeds = [b"data"],
        bump
    )]
    pub data: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeDiscriminator<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"data"],
        bump
    )]
    pub data: Account<'info, Data>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Realloc<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        realloc = 2000,
        realloc::payer = signer,
        realloc::zero = true,
        seeds = [b"data"],
        bump
    )]
    pub data: Account<'info, NewData>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Data {
    pub field1: u64,
    pub field2: u64,
    pub field3: u64,
    pub field4: Vec<u8>,
}

#[account]
#[derive(Default)]
pub struct NewData {
    pub field1: u64,
    pub field2: u64,
    pub field3: u64,
    pub field4: Vec<u8>,
    pub field5: u64,
}
