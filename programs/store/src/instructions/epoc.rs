use anchor_lang::prelude::*;
use crate::state::epoc::Epoc;
use crate::state::store::Store;

use crate::config::EPOC_TAG;

pub fn init_epoc(
  ctx: Context<InitEpoc>,
  id: i16,
  price: u64,
  total_supply: u128,
) -> Result<()> {
  let epoc = &mut ctx.accounts.epoc;
  epoc.init(id, price, total_supply)
}

pub fn set_epoc_price(
  ctx: Context<SetEpocPrice>,
  price: u64,
) -> Result<()> {
  let epoc = &mut ctx.accounts.epoc;
  epoc.set_price(price)
}

pub fn set_epoc_supply(
  ctx: Context<SetEpocSupply>,
  total_supply: u128
) -> Result<()> {
  let epoc = &mut ctx.accounts.epoc;
  epoc.set_total_supply(total_supply)
}

pub fn enable_epoc(
  ctx: Context<SetEpocEnabled>,
) -> Result<()> {
  let epoc = &mut ctx.accounts.epoc;
  epoc.set_enable().unwrap();

  let store = &mut ctx.accounts.store;
  store.set_epoc(epoc.get_id())
}

pub fn disable_epoc(
  ctx: Context<SetEpocDisabled>,
) -> Result<()> {
  let epoc = &mut ctx.accounts.epoc;
  epoc.set_disable()
}

#[derive(Accounts)]
#[instruction(id: i16)]
pub struct InitEpoc<'info> {
  #[account(
    init,
    payer = payer,
    space = 8 + Epoc::MAX_SIZE,
    seeds = [
      EPOC_TAG,
      b"_",
      &id.to_le_bytes()
    ],
    bump,
  )]
  pub epoc: Account<'info, Epoc>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct SetEpocPrice<'info> {
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(total_supply: u128)]
pub struct SetEpocSupply<'info> {
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetEpocEnabled<'info> {
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetEpocDisabled<'info> {
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}